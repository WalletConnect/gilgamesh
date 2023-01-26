pub use {
    super::StoreError,
    async_trait::async_trait,
    serde::{Deserialize, Serialize},
    std::sync::Arc,
    wither::{
        bson::{self, doc, oid::ObjectId},
        mongodb::{
            options::{ClientOptions, FindOptions},
            Client, Database,
        },
        Model,
    },
};
use {
    crate::config::Configuration,
    wither::mongodb::options::FindOneAndUpdateOptions,
};

pub type MessagesPersistentStorageArc = Arc<dyn MessagesPersistentStorage + Send + Sync + 'static>;

#[async_trait]
pub trait MessagesPersistentStorage: 'static + std::fmt::Debug + Send + Sync {
    async fn upsert_message(&self, message_id: &str) -> Result<(), StoreError>;
}

#[derive(Debug, Model, Serialize, Deserialize, PartialEq, Eq)]
#[model(
    collection_name = "Messages",
    index(keys = r#"doc!{"message_id": 1}"#, options = r#"doc!{"unique": true}"#),
    // index(
    //     Messages = r#"doc!{"identities.identity_key": 1}"#,
    //     options = r#"doc!{"unique": true}"#
    // )
)]
struct MongoMessages {
    /// Mongo's default `_id` field.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub message_id: String,

    // /// The account in CAIP-10 account identifier associated and controlled with
    // /// a blockchain private key. I.e. eip155:1:
    // /// 0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826.
    // pub account: String,

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
    async fn upsert_message(&self, message_id: &str) -> Result<(), StoreError> {
        let filter = doc! {
            "message_id": &message_id,
        };

        let update = doc! {
            "$set": {
                "message_id": &message_id,
            }
        };

        match MongoMessages::find_one_and_update(&self.db, filter, update, None).await? {
            Some(_) => Ok(()),
            None => Err(StoreError::NotFound(
                "Message".to_string(),
                message_id.to_string(),
            )),
        }
    }
}
