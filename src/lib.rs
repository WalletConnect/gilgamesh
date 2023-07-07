use {
    crate::{
        log::prelude::*,
        state::{MessagesStorageArc, RegistrationStorageArc},
    },
    axum::{
        http,
        routing::{get, post},
        Router,
    },
    config::Configuration,
    opentelemetry::{sdk::Resource, KeyValue},
    state::AppState,
    std::{net::SocketAddr, sync::Arc},
    store::mongo::MongoStore,
    tokio::{select, sync::broadcast},
    tower::ServiceBuilder,
    tower_http::{
        cors::{Any, CorsLayer},
        trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    },
};

pub mod auth;
pub mod config;
pub mod error;
pub mod handlers;
pub mod log;
pub mod macros;
pub mod metrics;
pub mod relay;
pub mod state;
pub mod store;
pub mod tags;

#[derive(Default)]
pub struct Options {
    pub messages_store: Option<MessagesStorageArc>,
    pub registration_store: Option<RegistrationStorageArc>,
}

pub async fn bootstrap(
    mut shutdown: broadcast::Receiver<()>,
    config: Configuration,
    options: Options,
) -> error::Result<()> {
    // Check config is valid and then throw the error if its not
    config.is_valid()?;

    let (messages_store, registration_store) =
        match (options.messages_store, options.registration_store) {
            (Some(messages_store), Some(registration_store)) => {
                (messages_store, registration_store)
            }
            (Some(messages_store), None) => {
                let store = Arc::new(MongoStore::new(&config).await?);
                (messages_store, store as RegistrationStorageArc)
            }
            (None, Some(registration_store)) => {
                let store = Arc::new(MongoStore::new(&config).await?);
                (store as MessagesStorageArc, registration_store)
            }
            _ => {
                let store = Arc::new(MongoStore::new(&config).await?);
                (
                    store.clone() as MessagesStorageArc,
                    store as RegistrationStorageArc,
                )
            }
        };

    let mut state = AppState::new(config.clone(), messages_store, registration_store)?;

    if config.validate_signatures {
        // Fetch public key so it's cached for the first 6hrs
        let public_key = state.relay_client.public_key().await;
        if public_key.is_err() {
            warn!("Failed initial fetch of Relay's Public Key, this may prevent items validation.")
        }
    }

    if state.config.telemetry_prometheus_port.is_some() {
        state.set_metrics(metrics::Metrics::new(Resource::new(vec![
            KeyValue::new("service_name", "history-server"),
            KeyValue::new(
                "service_version",
                state.build_info.crate_info.version.clone().to_string(),
            ),
        ]))?);
    }

    let port = state.config.port;
    let private_port = state.config.telemetry_prometheus_port.unwrap_or(3001);

    let state_arc = Arc::new(state);

    let global_middleware = ServiceBuilder::new().layer(
        TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().include_headers(true))
            .on_request(DefaultOnRequest::new().level(config.log_level()))
            .on_response(
                DefaultOnResponse::new()
                    .level(config.log_level())
                    .include_headers(true),
            ),
    );

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers([http::header::CONTENT_TYPE, http::header::AUTHORIZATION]);

    let app = Router::new()
        .route("/health", get(handlers::health::handler))
        .route("/messages", get(handlers::get_messages::handler))
        .route("/messages", post(handlers::save_message::handler))
        .route("/register", get(handlers::get_registration::handler))
        .route("/register", post(handlers::register::handler))
        .layer(global_middleware)
        .layer(cors)
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
