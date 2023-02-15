use {
    gilgamesh::{config::Configuration, stores::messages::MongoPersistentStorage},
    std::env,
};

pub struct PersistentStorage {
    pub store: MongoPersistentStorage,
}

impl PersistentStorage {
    pub async fn init() -> Self {
        let mongo_address = match env::var("MONGO_ADDRESS") {
            Ok(val) => val,
            Err(_) => "mongodb://admin:admin@mongo:27017/gilgamesh?authSource=admin".into(),
        };

        let config: Configuration = Configuration {
            port: 0,
            log_level: "INFO".into(),
            telemetry_enabled: None,
            telemetry_grpc_url: None,
            is_test: true,
            mongo_address,
        };

        let storage = MongoPersistentStorage::new(&config).await.unwrap();

        Self { store: storage }
    }

    pub async fn shutdown(&mut self) {}
}
