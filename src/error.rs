use {
    crate::{
        handlers::{ErrorField, ErrorLocation, ResponseError},
        log::prelude::*,
        relay::signature::{SIGNATURE_HEADER_NAME, TIMESTAMP_HEADER_NAME},
        store::StoreError,
    },
    axum::response::{IntoResponse, Response},
    hyper::StatusCode,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    #[error(transparent)]
    Envy(#[from] envy::Error),

    #[error(transparent)]
    Trace(#[from] opentelemetry::trace::TraceError),

    #[error(transparent)]
    Metrics(#[from] opentelemetry::metrics::MetricsError),

    #[error(transparent)]
    Prometheus(#[from] prometheus_core::Error),

    #[error(transparent)]
    Database(#[from] wither::mongodb::error::Error),

    #[error(transparent)]
    Hex(#[from] hex::FromHexError),

    #[error(transparent)]
    Ed25519(#[from] ed25519_dalek::ed25519::Error),

    #[error(transparent)]
    HttpRequest(#[from] reqwest::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Store(#[from] StoreError),

    #[error(transparent)]
    ToStr(#[from] axum::http::header::ToStrError),

    #[error("the `{0}` field must not be empty")]
    EmptyField(String),

    #[error("a required environment variable cannot be found")]
    RequiredEnvNotFound,

    #[error("timestamp header cannot not found")]
    MissingTimestampHeader,

    #[error("signature header cannot not found")]
    MissingSignatureHeader,

    #[error("middleware T::from_request failed")]
    FromRequestError,

    #[error("middleware failed to parse body")]
    ToBytesError,

    #[error("neither signature or timestamp header cannot not found")]
    MissingAllSignatureHeader,

    #[error("invalid configuration: {0}")]
    InvalidConfiguration(String),

    #[error("invalid options provided for {0}")]
    InvalidOptionsProvided(String),

    #[error("invalid update request")]
    InvalidUpdateRequest,

    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    #[error("History item received without a topic, please ensure all required  parameters set")]
    MissingTopic,

    #[error("this should not have occurred; used when case has been handled before")]
    InternalServerError,

    #[error(transparent)]
    JwtError(#[from] relay_rpc::jwt::JwtError),

    #[error(transparent)]
    AuthError(#[from] relay_rpc::auth::Error),

    #[error("the provided authentication does not authenticate the request")]
    InvalidAuthentication,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        error!("responding with error ({:?})", self);
        match self {
            Error::Database(e) => crate::handlers::Response::new_failure(StatusCode::INTERNAL_SERVER_ERROR, vec![
                ResponseError {
                    name: "mongodb".to_string(),
                    message: e.to_string(),
                }
            ], vec![]),
            Error::Hex(e) => crate::handlers::Response::new_failure(StatusCode::INTERNAL_SERVER_ERROR, vec![
                ResponseError {
                    name: "from_hex".to_string(),
                    message: e.to_string(),
                }
            ], vec![]),
            Error::Ed25519(e) => crate::handlers::Response::new_failure(StatusCode::INTERNAL_SERVER_ERROR, vec![
                ResponseError {
                    name: "ed25519".to_string(),
                    message: e.to_string(),
                }
            ], vec![]),
            Error::HttpRequest(e) => crate::handlers::Response::new_failure(StatusCode::INTERNAL_SERVER_ERROR, vec![
                ResponseError {
                    name: "http_request".to_string(),
                    message: e.to_string(),
                }
            ], vec![]),
            Error::Store(e) => match e {
                StoreError::Database(e) => crate::handlers::Response::new_failure(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    vec![ResponseError {
                        name: "mongodb".to_string(),
                        message: e.to_string(),
                    }],
                    vec![],
                ),
                StoreError::NotFound(entity, id) => crate::handlers::Response::new_failure(
                    StatusCode::NOT_FOUND,
                    vec![],
                    vec![ErrorField {
                        field: format!("{}.id", &entity),
                        description: format!("Cannot find {entity} with specified identifier {id}"),
                        location: ErrorLocation::Body, // TODO evaluate if correct location
                    }],
                ),
            },
            Error::MissingAllSignatureHeader => crate::handlers::Response::new_failure(StatusCode::UNAUTHORIZED, vec![
                ResponseError {
                    name: "history_item_validation_failed".to_string(),
                    message: "Failed to validate history item, please ensure that all required headers are provided.".to_string(),
                }
            ], vec![
                ErrorField {
                    field: SIGNATURE_HEADER_NAME.to_string(),
                    description: "Missing signature".to_string(),
                    location: ErrorLocation::Header
                },
                ErrorField {
                    field: TIMESTAMP_HEADER_NAME.to_string(),
                    description: "Missing timestamp".to_string(),
                    location: ErrorLocation::Header
                }
            ]),
            Error::MissingSignatureHeader => crate::handlers::Response::new_failure(StatusCode::UNAUTHORIZED, vec![
                ResponseError {
                    name: "history_item_validation_failed".to_string(),
                    message: "Failed to validate history item, please ensure that all required headers are provided.".to_string(),
                }
            ], vec![
                ErrorField {
                    field: SIGNATURE_HEADER_NAME.to_string(),
                    description: "Missing signature".to_string(),
                    location: ErrorLocation::Header
                }
            ]),
            Error::MissingTimestampHeader => crate::handlers::Response::new_failure(StatusCode::UNAUTHORIZED, vec![
                ResponseError {
                    name: "history_item_validation_failed".to_string(),
                    message: "Failed to validate history item, please ensure that all required headers are provided.".to_string(),
                }
            ], vec![
                ErrorField {
                    field: TIMESTAMP_HEADER_NAME.to_string(),
                    description: "Missing timestamp".to_string(),
                    location: ErrorLocation::Header
                }
            ]),
            Error::MissingTopic => crate::handlers::Response::new_failure(
                StatusCode::BAD_REQUEST,
                vec![ResponseError {
                    name: "topic".to_string(),
                    message: "encrypted push notifications require topic to be set".to_string(),
                }],
                vec![],
            ),
            Error::InvalidUpdateRequest => crate::handlers::Response::new_failure(
                StatusCode::BAD_REQUEST,
                vec![ResponseError {
                    name: "appendTags/removeTags".to_string(),
                    message: "cannot append and remove the same tag".to_string(),
                }],
                vec![],
            ),
            e => crate::handlers::Response::new_failure(StatusCode::INTERNAL_SERVER_ERROR, vec![
                ResponseError {
                    name: "unknown_error".to_string(),
                    message: "This error should not have occurred. Please file an issue at: https://github.com/walletconnect/echo-server".to_string(),
                },
                ResponseError {
                    name: "dbg".to_string(),
                    message: format!("{e:?}"),
                }
            ], vec![])
        }.into_response()
    }
}
