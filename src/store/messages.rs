use {
    super::StoreError,
    async_trait::async_trait,
    serde::{Deserialize, Serialize},
    wither::{
        bson::{self, doc, oid::ObjectId},
        Model,
    },
};

#[derive(Debug, Model, Serialize, Deserialize, PartialEq, Eq)]
#[model(
    collection_name = "Messages",
    index(keys = r#"doc!{"ts": 1}"#),
    index(keys = r#"doc!{"ts": -1}"#),
    index(keys = r#"doc!{"topic": 1}"#),
    index(keys = r#"doc!{"message_id": 1}"#, options = r#"doc!{"unique": true}"#)
)]
pub struct Message {
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
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StoreMessages {
    pub messages: Vec<Message>,
    pub next_id: Option<String>,
}

#[async_trait]
pub trait MessagesStore: 'static + std::fmt::Debug + Send + Sync {
    async fn upsert_message(&self, message_id: &str, topic: &str) -> Result<(), StoreError>;
    async fn get_messages_after(
        &self,
        topic: &str,
        origin: Option<&str>,
        message_count: usize,
    ) -> Result<StoreMessages, StoreError>;
    async fn get_messages_before(
        &self,
        topic: &str,
        origin: Option<&str>,
        message_count: usize,
    ) -> Result<StoreMessages, StoreError>;
}
