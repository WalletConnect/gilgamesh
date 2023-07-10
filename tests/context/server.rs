use {
    crate::storage::mocks::{messages::MockMessageStore, registrations::MockRegistrationStore},
    gilgamesh::{config::Configuration, Options},
    std::{
        env,
        net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener},
        sync::Arc,
    },
    tokio::{
        runtime::Handle,
        sync::broadcast,
        time::{sleep, Duration},
    },
};

pub struct Gilgamesh {
    pub public_addr: SocketAddr,
    pub message_store: Arc<MockMessageStore>,
    pub registration_store: Arc<MockRegistrationStore>,
    shutdown_signal: broadcast::Sender<()>,
    is_shutdown: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {}

impl Gilgamesh {
    pub async fn start() -> Self {
        let public_port = get_random_port();
        let rt = Handle::current();
        let public_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), public_port);

        let (signal, shutdown) = broadcast::channel(1);

        let message_store = Arc::new(MockMessageStore::new());
        let registration_store = Arc::new(MockRegistrationStore::new());

        let options = Options {
            messages_store: Some(message_store.clone()),
            registration_store: Some(registration_store.clone()),
        };

        std::thread::spawn(move || {
            rt.block_on(async move {
                let public_port = public_port;
                let mongo_address = env::var("MONGO_ADDRESS").unwrap_or(
                    "mongodb://admin:admin@mongo:27017/gilgamesh?authSource=admin".into(),
                );

                let config: Configuration = Configuration {
                    port: public_port,
                    public_url: format!("http://127.0.0.1:{public_port}"),
                    log_level: "info,history-server=info".into(),
                    relay_url: "https://relay.walletconnect.com".into(),
                    validate_signatures: false,
                    mongo_address,
                    is_test: true,
                    otel_exporter_otlp_endpoint: None,
                    telemetry_prometheus_port: Some(get_random_port()),
                };

                gilgamesh::bootstrap(shutdown, config, options).await
            })
            .unwrap();
        });

        if let Err(e) = wait_for_server_to_start(public_port).await {
            panic!("Failed to start server with error: {e:?}")
        }

        Self {
            public_addr,
            message_store,
            registration_store,
            shutdown_signal: signal,
            is_shutdown: false,
        }
    }

    pub async fn shutdown(&mut self) {
        if self.is_shutdown {
            return;
        }
        self.is_shutdown = true;
        let _ = self.shutdown_signal.send(());
        wait_for_server_to_shutdown(self.public_addr.port())
            .await
            .unwrap();
    }
}

// Finds a free port.
pub fn get_random_port() -> u16 {
    use std::sync::atomic::{AtomicU16, Ordering};

    static NEXT_PORT: AtomicU16 = AtomicU16::new(9000);

    loop {
        let port = NEXT_PORT.fetch_add(1, Ordering::SeqCst);

        if is_port_available(port) {
            return port;
        }
    }
}

fn is_port_available(port: u16) -> bool {
    TcpListener::bind(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port)).is_ok()
}

async fn wait_for_server_to_shutdown(port: u16) -> crate::ErrorResult<()> {
    let poll_fut = async {
        while !is_port_available(port) {
            sleep(Duration::from_millis(10)).await;
        }
    };

    Ok(tokio::time::timeout(Duration::from_secs(3), poll_fut).await?)
}

async fn wait_for_server_to_start(port: u16) -> crate::ErrorResult<()> {
    let poll_fut = async {
        while is_port_available(port) {
            sleep(Duration::from_millis(10)).await;
        }
    };

    Ok(tokio::time::timeout(Duration::from_secs(5), poll_fut).await?)
}
