use {
    crate::{
        error,
        metrics::Metrics,
        relay::RelayClient,
        store::messages::MessagesStore,
        Configuration,
    },
    build_info::BuildInfo,
    std::sync::Arc,
};

pub type MessagesStorageArc = Arc<dyn MessagesStore + Send + Sync + 'static>;

pub trait State {
    fn config(&self) -> Configuration;
    fn build_info(&self) -> BuildInfo;
    fn messages_store(&self) -> MessagesStorageArc;
    fn relay_client(&self) -> RelayClient;
    fn validate_signatures(&self) -> bool;
}

#[derive(Clone)]
pub struct AppState {
    pub config: Configuration,
    pub build_info: BuildInfo,
    pub metrics: Option<Metrics>,
    pub messages_store: MessagesStorageArc,
    pub relay_client: RelayClient,
}

build_info::build_info!(fn build_info);

impl AppState {
    pub fn new(
        config: Configuration,
        messages_store: MessagesStorageArc,
    ) -> error::Result<AppState> {
        let build_info: &BuildInfo = build_info();

        let relay_url = config.relay_url.to_string();

        Ok(AppState {
            config,
            build_info: build_info.clone(),
            metrics: None,
            messages_store,
            relay_client: RelayClient::new(relay_url),
        })
    }

    pub fn set_metrics(&mut self, metrics: Metrics) {
        self.metrics = Some(metrics);
    }
}

impl State for Arc<AppState> {
    fn config(&self) -> Configuration {
        self.config.clone()
    }

    fn build_info(&self) -> BuildInfo {
        self.build_info.clone()
    }

    fn messages_store(&self) -> MessagesStorageArc {
        self.messages_store.clone()
    }

    fn relay_client(&self) -> RelayClient {
        self.relay_client.clone()
    }

    fn validate_signatures(&self) -> bool {
        self.config.validate_signatures
    }
}
