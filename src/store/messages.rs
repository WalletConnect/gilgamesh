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
    index(keys = r#"doc!{"client_id": 1, "topic": 1}"#),
    index(
        keys = r#"doc!{"client_id": 1, "topic": 1, "message_id": 1}"#,
        options = r#"doc!{"unique": true}"#
    )
)]
pub struct Message {
    /// MongoDB's default `_id` field.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    /// The number of milliseconds since Epoch
    #[serde(rename = "ts")]
    pub timestamp: bson::DateTime,
    /// The message's client ID.
    pub client_id: String,
    /// The message's topic ID.
    pub topic: String,
    /// The SHA256 of the message.
    pub message_id: String,
    /// The actual message.
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StoreMessages {
    pub messages: Vec<Message>,
    pub next_id: Option<String>,
}

#[async_trait]
pub trait MessagesStore: 'static + std::fmt::Debug + Send + Sync {
    async fn upsert_message(
        &self,
        client_id: &str,
        message_id: &str,
        topic: &str,
        message: &str,
    ) -> Result<(), StoreError>;
    async fn get_messages_after(
        &self,
        client_id: &str,
        topic: &str,
        origin: Option<&str>,
        message_count: usize,
    ) -> Result<StoreMessages, StoreError>;
    async fn get_messages_before(
        &self,
        client_id: &str,
        topic: &str,
        origin: Option<&str>,
        message_count: usize,
    ) -> Result<StoreMessages, StoreError>;
}
