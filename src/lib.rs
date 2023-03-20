use {
    crate::log::prelude::*,
    axum::{
        routing::{get, post},
        Router,
    },
    config::Configuration,
    opentelemetry::{sdk::Resource, KeyValue},
    state::{AppState, MessagesStorageArc},
    std::{net::SocketAddr, sync::Arc},
    store::messages::MongoStore,
    tokio::{select, sync::broadcast},
    tower::ServiceBuilder,
    tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    tracing::Level,
};

pub mod config;
pub mod error;
pub mod handlers;
pub mod log;
pub mod macros;
pub mod metrics;
pub mod middleware;
pub mod relay;
pub mod state;
pub mod store;

pub async fn bootstrap(
    mut shutdown: broadcast::Receiver<()>,
    config: Configuration,
) -> error::Result<()> {
    // Check config is valid and then throw the error if its not
    config.is_valid()?;

    let store: MessagesStorageArc = Arc::new(MongoStore::new(&config).await?);
    let mut state = AppState::new(config.clone(), store)?;

    // Fetch public key so it's cached for the first 6hrs
    let public_key = state.relay_client.public_key().await;
    if public_key.is_err() {
        warn!("Failed initial fetch of Relay's Public Key, this may prevent webhook validation.")
    }

    if state.config.telemetry_prometheus_port.is_some() {
        state.set_metrics(metrics::Metrics::new(Resource::new(vec![
            KeyValue::new("service_name", "echo-server"),
            KeyValue::new(
                "service_version",
                state.build_info.crate_info.version.clone().to_string(),
            ),
        ]))?);
    }

    let port = state.config.port;
    let private_port = state.config.telemetry_prometheus_port.unwrap_or(3001);

    let _allowed_origins = state.config.cors_allowed_origins.clone();

    let state_arc = Arc::new(state);

    let global_middleware = ServiceBuilder::new().layer(
        TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().include_headers(true))
            .on_request(DefaultOnRequest::new().level(Level::INFO))
            .on_response(
                DefaultOnResponse::new()
                    .level(Level::INFO)
                    .include_headers(true),
            ),
    );

    let app = Router::new()
        .route("/health", get(handlers::health::handler))
        .route("/messages", get(handlers::get_messages::handler))
        .route("/messages", post(handlers::save_message::handler))
        .layer(global_middleware)
        .with_state(state_arc.clone());

    let private_app = Router::new()
        .route("/metrics", get(handlers::metrics::handler))
        .with_state(state_arc);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let private_addr = SocketAddr::from(([0, 0, 0, 0], private_port));

    select! {
        _ = axum::Server::bind(&addr).serve(app.into_make_service()) => info!("Server terminating"),
        _ = axum::Server::bind(&private_addr).serve(private_app.into_make_service()) => info!("Internal Server terminating"),
        _ = shutdown.recv() => info!("Shutdown signal received, killing servers"),
    }

    Ok(())
}
