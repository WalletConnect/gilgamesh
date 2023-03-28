use {
    crate::{
        error,
        metrics::Metrics,
        relay::RelayClient,
        store::{messages::MessagesStore, registrations::RegistrationStore},
        Configuration,
    },
    build_info::BuildInfo,
    moka::future::Cache,
    std::{collections::HashSet, sync::Arc, time::Duration},
};

pub type MessagesStorageArc = Arc<dyn MessagesStore + Send + Sync + 'static>;
pub type RegistrationStorageArc = Arc<dyn RegistrationStore + Send + Sync + 'static>;

#[derive(Clone)]
pub struct CachedRegistration {
    pub tags: Vec<Arc<str>>,
    pub relay_url: Arc<str>,
}

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
    pub registration_store: RegistrationStorageArc,
    pub registration_cache: Cache<Arc<str>, CachedRegistration>,
    pub relay_client: RelayClient,
    pub auth_aud: HashSet<String>,
}

build_info::build_info!(fn build_info);

impl AppState {
    pub fn new(
        config: Configuration,
        messages_store: MessagesStorageArc,
        registration_store: RegistrationStorageArc,
    ) -> error::Result<AppState> {
        let build_info: &BuildInfo = build_info();

        let relay_url = config.relay_url.to_string();

        let registration_cache = Cache::builder()
            .weigher(|_key, value: &CachedRegistration| -> u32 {
                value.relay_url.len().try_into().unwrap_or(u32::MAX)
                    + value
                        .tags
                        .iter()
                        .fold(0, |acc, tag| acc + (tag.len() as u32))
            })
            .max_capacity(32 * 1024 * 1024)
            .time_to_live(Duration::from_secs(30 * 60))
            .time_to_idle(Duration::from_secs(5 * 60))
            .build();

        Ok(AppState {
            config,
            build_info: build_info.clone(),
            metrics: None,
            messages_store,
            registration_store,
            registration_cache,
            relay_client: RelayClient::new(relay_url),
            auth_aud: [
                "wss://relay.walletconnect.com".to_owned(),
                "https://history.walletconnect.com".to_owned(),
            ]
            .into(),
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
