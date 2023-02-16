use {
    crate::{handlers::ResponseError, stores::StoreError},
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
    Store(#[from] crate::stores::StoreError),

    #[error(transparent)]
    Database(#[from] wither::mongodb::error::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::Database(e) => crate::handlers::Response::new_failure(
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseError {
                    name: "mongodb".to_string(),
                    message: e.to_string(),
                },
            ),
            Error::Store(e) => match e {
                StoreError::Database(e) => crate::handlers::Response::new_failure(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ResponseError {
                        name: "mongodb".to_string(),
                        message: e.to_string(),
                    },
                ),
                StoreError::NotFound(entity, id) => crate::handlers::Response::new_failure(
                    StatusCode::NOT_FOUND,
                    ResponseError {
                        name: format!("{} not found", &entity),
                        message: format!("Cannot find {entity} with specified identifier {id}"),
                    },
                ),
            },
            _ => crate::handlers::Response::new_failure(
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseError {
                    name: "unknown_error".to_string(),
                    message: "This error should not have occurred. Please file an issue at: https://github.com/walletconnect/gilgamesh".to_string(),
                }
            ),
        }.into_response()
    }
}
