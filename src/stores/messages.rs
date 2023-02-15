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
    chrono::Utc,
    futures::TryStreamExt,
    wither::{bson::Document, mongodb::options::FindOneAndUpdateOptions},
};

pub type MessagesPersistentStorageArc = Arc<dyn MessagesPersistentStorage + Send + Sync + 'static>;

pub struct GetMessagesResponse {
    pub messages: Vec<MongoMessages>,
    pub next_id: Option<String>,
}

#[async_trait]
pub trait MessagesPersistentStorage: 'static + std::fmt::Debug + Send + Sync {
    async fn upsert_message(&self, message_id: &str, topic: &str) -> Result<(), StoreError>;
    async fn get_messages_after(
        &self,
        topic: &str,
        origin: Option<&str>,
        message_count: usize,
    ) -> Result<GetMessagesResponse, StoreError>;
    async fn get_messages_before(
        &self,
        topic: &str,
        origin: Option<&str>,
        message_count: usize,
    ) -> Result<GetMessagesResponse, StoreError>;
}

#[derive(Debug, Model, Serialize, Deserialize, PartialEq, Eq)]
#[model(
    collection_name = "Messages",
    index(keys = r#"doc!{"ts": 1}"#),
    index(keys = r#"doc!{"ts": -1}"#),
    index(keys = r#"doc!{"topic": 1}"#),
    index(keys = r#"doc!{"message_id": 1}"#, options = r#"doc!{"unique": true}"#)
)]
pub struct MongoMessages {
    /// MongoDB's default `_id` field.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    /// The number of milliseconds since Epoch
    #[serde(rename = "ts")]
    pub timestamp: bson::DateTime,
    /// The message's topic ID.
    pub topic: String,
    /// The SHA256 of the message.
    pub message_id: String,
    // TODO describe identities
    // pub identities: Vec<MongoIdentity>,≤

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

    async fn get_message_timestamp(
        &self,
        topic: &str,
        message_id: &str,
    ) -> Result<bson::DateTime, StoreError> {
        let filter = doc! {
            "topic": &topic,
            "message_id": message_id,
        };

        let cursor = MongoMessages::find(&self.db, filter, None).await?;
        let origin: Vec<MongoMessages> = cursor.try_collect().await?;
        let origin = origin.first().ok_or(StoreError::NotFound(
            topic.to_string(),
            message_id.to_string(),
        ))?;

        Ok(origin.timestamp)
    }

    async fn get_messages(
        &self,
        topic: &str,
        origin: Option<&str>,
        message_count: usize,
        comparator: &str,
        sort_order: i32,
    ) -> Result<GetMessagesResponse, StoreError> {
        let filter: Result<Document, StoreError> = match origin {
            None => Ok(doc! {
                "topic": &topic,
            }),
            Some(origin) => {
                let ts = self.get_message_timestamp(topic, origin).await?;
                Ok(doc! {
                    "topic": &topic,
                    "ts": { comparator: ts }
                })
            }
        };
        let filter = filter?;

        let message_count: i64 = message_count as i64;
        let limit = -(message_count + 1);
        let options = FindOptions::builder()
            .sort(doc! {"ts": sort_order})
            .limit(limit)
            .build();

        let cursor = MongoMessages::find(&self.db, filter, options).await?;

        let mut messages: Vec<MongoMessages> = cursor.try_collect().await?;

        if messages.len() > message_count as usize {
            let next_id = messages.pop().map(|message| message.message_id);
            return Ok(GetMessagesResponse { messages, next_id });
        }

        Ok(GetMessagesResponse {
            messages,
            next_id: None,
        })
    }
}

#[async_trait]
impl MessagesPersistentStorage for MongoPersistentStorage {
    async fn upsert_message(&self, topic: &str, message_id: &str) -> Result<(), StoreError> {
        let filter = doc! {
            "message_id": &message_id,
        };

        let update = doc! {
            "$set": {
                "ts": Utc::now(),
                "topic": &topic,
                "message_id": &message_id,
            }
        };

        let option = FindOneAndUpdateOptions::builder().upsert(true).build();

        match MongoMessages::find_one_and_update(&self.db, filter, update, option).await? {
            Some(_) => Ok(()),
            None => Ok(()),
        }
    }

    async fn get_messages_after(
        &self,
        topic: &str,
        origin: Option<&str>,
        message_count: usize,
    ) -> Result<GetMessagesResponse, StoreError> {
        self.get_messages(topic, origin, message_count, "$gte", 1)
            .await
    }

    async fn get_messages_before(
        &self,
        topic: &str,
        origin: Option<&str>,
        message_count: usize,
    ) -> Result<GetMessagesResponse, StoreError> {
        self.get_messages(topic, origin, message_count, "$lte", -1)
            .await
    }
}
