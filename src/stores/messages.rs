pub use {
    super::StoreError,
    async_trait::async_trait,
    serde::{Deserialize, Serialize},
    std::sync::Arc,
    wither::{
        bson::{self, doc, oid::ObjectId},
        mongodb::{
            options::{ClientOptions, FindOptions},
            Client,
            Database,
        },
        Model,
    },
};
use {
    crate::config::Configuration,
    futures::TryStreamExt,
    wither::mongodb::options::FindOneAndUpdateOptions,
};

pub type MessagesPersistentStorageArc = Arc<dyn MessagesPersistentStorage + Send + Sync + 'static>;

#[async_trait]
pub trait MessagesPersistentStorage: 'static + std::fmt::Debug + Send + Sync {
    async fn upsert_message(&self, message_id: &str, topic: &str) -> Result<(), StoreError>;
    async fn get_messages(&self, topic: &str) -> Result<Vec<MongoMessages>, StoreError>;
}

#[derive(Debug, Model, Serialize, Deserialize, PartialEq, Eq)]
#[model(
    collection_name = "Messages",
    index(keys = r#"doc!{"message_id": 1}"#, options = r#"doc!{"unique": true}"#),
    index(keys = r#"doc!{"topic": 1}"#)
)]
pub struct MongoMessages {
    /// Mongo's default `_id` field.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub message_id: String,
    pub topic: String,
    // /// TODO describe identities
    // pub identities: Vec<MongoIdentity>,

    // /// The proposal encryption key used by a peer client to derive a shared DH
    // /// symmetric key to encrypt proposals.
    // pub invite_key: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MongoPersistentStorage {
    db: Database,
}

impl MongoPersistentStorage {
    pub async fn new(config: &Configuration) -> anyhow::Result<Self> {
        let url = &config.mongo_address;

        let client_options = ClientOptions::parse(url).await?;
        let client = Client::with_options(client_options)?;
        let db = client.default_database().ok_or_else(|| {
            anyhow::anyhow!("no default database specified in the connection URL")
        })?;

        MongoMessages::sync(&db).await?;

        Ok(Self { db })
    }
}

#[async_trait]
impl MessagesPersistentStorage for MongoPersistentStorage {
    async fn upsert_message(&self, message_id: &str, topic: &str) -> Result<(), StoreError> {
        let filter = doc! {
            "message_id": &message_id,
        };

        let update = doc! {
            "$set": {
                "message_id": &message_id,
                "topic": &topic,
            }
        };

        let option = FindOneAndUpdateOptions::builder().upsert(true).build();

        match MongoMessages::find_one_and_update(&self.db, filter, update, option).await? {
            Some(_) => Ok(()),
            None => Ok(()),
        }
    }

    async fn get_messages(&self, topic: &str) -> Result<Vec<MongoMessages>, StoreError> {
        let filter = doc! {
            "topic": &topic,
        };

        let options = FindOptions::builder().sort(doc! {"message_id": 1}).build();

        let cursor = MongoMessages::find(&self.db, filter, options).await?;

        let messages: Vec<MongoMessages> = cursor.try_collect().await?;

        Ok(messages)
    }
}
