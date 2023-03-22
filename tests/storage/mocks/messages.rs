use {
    async_trait::async_trait,
    gilgamesh::store::{
        messages::{Message, MessagesStore, StoreMessages},
        StoreError,
    },
    std::fmt::Debug,
    tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender},
};

#[derive(Debug)]
pub struct MockMessageStore {
    // pub messages: Arc<Mutex<Vec<Message>>>,
    pub sender: UnboundedSender<Message>,
    pub receiver: UnboundedReceiver<Message>,
}

impl MockMessageStore {
    pub fn new() -> Self {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        Self {
            // messages: Arc::new(Mutex::new(Vec::new())),
            sender,
            receiver,
        }
    }
}

#[async_trait]
impl MessagesStore for MockMessageStore {
    async fn upsert_message(
        &self,
        _client_id: &str,
        _message_id: &str,
        _topic: &str,
        _message: &str,
    ) -> Result<(), StoreError> {
        todo!()
    }

    async fn get_messages_after(
        &self,
        _client_id: &str,
        _topic: &str,
        _origin: Option<&str>,
        _message_count: usize,
    ) -> Result<StoreMessages, StoreError> {
        todo!()
    }

    async fn get_messages_before(
        &self,
        _client_id: &str,
        _topic: &str,
        _origin: Option<&str>,
        _message_count: usize,
    ) -> Result<StoreMessages, StoreError> {
        todo!()
    }
}
