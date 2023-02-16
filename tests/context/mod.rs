use {
    self::{server::Gilgamesh, store::PersistentStorage},
    async_trait::async_trait,
    test_context::AsyncTestContext,
};

mod server;
mod store;

pub struct ServerContext {
    pub server: Gilgamesh,
}

#[async_trait]
impl AsyncTestContext for ServerContext {
    async fn setup() -> Self {
        let server = Gilgamesh::start().await;
        Self { server }
    }

    async fn teardown(mut self) {
        self.server.shutdown().await;
    }
}

#[derive(Clone)]
pub struct StoreContext {
    pub storage: PersistentStorage,
}

#[async_trait]
impl AsyncTestContext for StoreContext {
    async fn setup() -> Self {
        let storage = PersistentStorage::init().await;
        Self { storage }
    }

    async fn teardown(mut self) {
        self.storage.shutdown().await;
    }
}
