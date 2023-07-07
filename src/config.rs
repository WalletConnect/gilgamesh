use {super::error, serde::Deserialize, std::str::FromStr};

const DEFAULT_PORT_NUMBER: u16 = 3001;
const DEFAULT_LOG_LEVEL: &str = "WARN";
const DEFAULT_RELAY_URL: &str = "https://relay.walletconnect.com";
const DEFAULT_VALIDATE_SIGNATURES: bool = true;

/// The server configuration.
#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Configuration {
    /// The port number of the HTTP server.
    #[serde(default = "default_port")]
    pub port: u16,
    pub public_url: String,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    /// The URL of the Relay server.
    #[serde(default = "default_relay_url")]
    pub relay_url: String,
    /// A flag to enable or disable the signature validation.
    #[serde(default = "default_validate_signatures")]
    pub validate_signatures: bool,
    /// The address of the MongoDB instance.
    pub mongo_address: String,
    /// An internal flag to disable logging, cannot be defined by user.
    #[serde(default = "default_is_test", skip)]
    pub is_test: bool,

    pub otel_exporter_otlp_endpoint: Option<String>,
    pub telemetry_prometheus_port: Option<u16>,
}

impl Configuration {
    /// Validate the configuration.
    pub fn is_valid(&self) -> error::Result<()> {
        Ok(())
    }

    pub fn log_level(&self) -> tracing::Level {
        tracing::Level::from_str(self.log_level.as_str()).unwrap_or(tracing::Level::INFO)
    }
}

fn default_port() -> u16 {
    DEFAULT_PORT_NUMBER
}

fn default_log_level() -> String {
    DEFAULT_LOG_LEVEL.to_string()
}

fn default_relay_url() -> String {
    DEFAULT_RELAY_URL.to_string()
}

fn default_validate_signatures() -> bool {
    DEFAULT_VALIDATE_SIGNATURES
}

fn default_is_test() -> bool {
    false
}

/// Create a new configuration from the environment variables.
pub fn get_config() -> error::Result<Configuration> {
    let config = envy::from_env::<Configuration>()?;
    Ok(config)
}
