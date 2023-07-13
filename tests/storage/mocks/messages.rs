use {
    archive::store::{
        messages::{Message, MessagesStore, StoreMessages},
        StoreError,
    },
    async_trait::async_trait,
    chrono::Utc,
    moka::future::Cache,
    std::{fmt::Debug, sync::Arc},
};

#[derive(Debug)]
pub struct MockMessageStore {
    pub messages: Cache<String, Message>,
    pub client_id: Option<String>,
}

fn cache_key(client_id: &str, topic: &str, message_id: &str) -> String {
    format!("{client_id}:{topic}:{message_id}")
}

impl MockMessageStore {
    pub fn new() -> Self {
        Self {
            messages: Cache::builder().build(),
            client_id: None,
        }
    }

    pub async fn test_get(
        &self,
        client_id: &str,
        topic: &str,
        message_id: &str,
    ) -> Option<Message> {
        let key = cache_key(client_id, topic, message_id);
        self.messages.get(&key)
    }

    pub async fn test_add(&self, message: Message) {
        let key = cache_key(
            message.client_id.as_ref(),
            message.topic.as_ref(),
            message.message_id.as_ref(),
        );
        self.messages.insert(key, message).await;
    }

    pub fn test_get_messages(&self) -> Vec<Message> {
        self.messages.iter().map(|(_, v)| v).collect()
    }
}

#[async_trait]
impl MessagesStore for MockMessageStore {
    async fn upsert_message(
        &self,
        method: &str,
        client_id: &str,
        topic: &str,
        message_id: &str,
        message: &str,
    ) -> Result<(), StoreError> {
        self.test_add(Message {
            id: None,
            timestamp: Utc::now().into(),
            method: Arc::from(method),
            client_id: Arc::from(client_id),
            message_id: Arc::from(message_id),
            topic: Arc::from(topic),
            message: Arc::from(message),
        })
        .await;

        Ok(())
    }

    async fn get_messages_after(
        &self,
        _topic: &str,
        _origin: Option<&str>,
        _message_count: usize,
    ) -> Result<StoreMessages, StoreError> {
        Ok(StoreMessages {
            messages: self.test_get_messages(),
            next_id: Some(Arc::from("after")),
        })
    }

    async fn get_messages_before(
        &self,
        _topic: &str,
        _origin: Option<&str>,
        _message_count: usize,
    ) -> Result<StoreMessages, StoreError> {
        Ok(StoreMessages {
            messages: self.test_get_messages(),
            next_id: Some(Arc::from("before")),
        })
    }
}
