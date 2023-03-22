use {
    async_trait::async_trait,
    chrono::Utc,
    gilgamesh::store::{
        messages::{Message, MessagesStore, StoreMessages},
        StoreError,
    },
    moka::future::Cache,
    std::fmt::Debug,
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
            message.client_id.as_str(),
            message.topic.as_str(),
            message.message_id.as_str(),
        );
        self.messages.insert(key, message).await;
    }

    pub fn test_get_keys(&self) -> Vec<String> {
        self.messages.iter().map(|(k, _)| k.to_string()).collect()
    }

    pub fn test_get_messages(&self) -> Vec<Message> {
        self.messages.iter().map(|(_, v)| v).collect()
    }
}

#[async_trait]
impl MessagesStore for MockMessageStore {
    async fn upsert_message(
        &self,
        client_id: &str,
        topic: &str,
        message_id: &str,
        message: &str,
    ) -> Result<(), StoreError> {
        self.test_add(Message {
            id: None,
            timestamp: Utc::now().into(),
            client_id: client_id.to_string(),
            message_id: message_id.to_string(),
            topic: topic.to_string(),
            message: message.to_string(),
        })
        .await;

        Ok(())
    }

    async fn get_messages_after(
        &self,
        client_id: &str,
        _topic: &str,
        _origin: Option<&str>,
        _message_count: usize,
    ) -> Result<StoreMessages, StoreError> {
        if self.client_id.is_some() && self.client_id != Some(client_id.to_string()) {
            return Err(StoreError::NotFound(
                "messages".to_string(),
                client_id.to_string(),
            ));
        }

        Ok(StoreMessages {
            messages: self.test_get_messages(),
            next_id: Some("after".to_string()),
        })
    }

    async fn get_messages_before(
        &self,
        client_id: &str,
        _topic: &str,
        _origin: Option<&str>,
        _message_count: usize,
    ) -> Result<StoreMessages, StoreError> {
        if self.client_id.is_some() && self.client_id != Some(client_id.to_string()) {
            return Err(StoreError::NotFound(
                "messages".to_string(),
                client_id.to_string(),
            ));
        }

        Ok(StoreMessages {
            messages: self.test_get_messages(),
            next_id: Some("before".to_string()),
        })
    }
}
