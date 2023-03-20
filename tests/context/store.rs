use {
    crate::context::server::get_random_port,
    gilgamesh::{config::Configuration, store::messages::MongoStore},
    std::env,
};

#[derive(Clone)]
pub struct PersistentStorage {
    pub store: MongoStore,
}

impl PersistentStorage {
    pub async fn init() -> Self {
        let public_port = get_random_port();
        let mongo_address = match env::var("MONGO_ADDRESS") {
            Ok(val) => val,
            Err(_) => "mongodb://admin:admin@mongo:27017/gilgamesh?authSource=admin".into(),
        };

        let config: Configuration = Configuration {
            port: public_port,
            public_url: format!("http://127.0.0.1:{public_port}"),
            log_level: "info,history-server=info".into(),
            relay_url: "https://relay.walletconnect.com".into(),
            validate_signatures: false,
            mongo_address,
            is_test: true,
            cors_allowed_origins: vec!["*".to_string()],
            otel_exporter_otlp_endpoint: None,
            telemetry_prometheus_port: Some(get_random_port()),
        };

        let storage = MongoStore::new(&config).await.unwrap();

        Self { store: storage }
    }

    pub async fn shutdown(&mut self) {}
}
