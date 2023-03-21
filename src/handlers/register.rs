use {
    crate::{
        auth::{jwt, AuthBearer},
        error,
        handlers::Response,
        increment_counter,
        log::prelude::*,
        state::AppState,
        store::messages::{Deserialize, Serialize},
    },
    axum::{extract::State, Json},
    std::sync::Arc,
};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RegisterPayload {
    pub tags: Vec<String>,
    pub relay_url: String,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    AuthBearer(token): AuthBearer,
    Json(body): Json<RegisterPayload>,
) -> error::Result<Response> {
    let _client_id = jwt::Jwt(token).decode(&state.auth_aud.clone())?;

    increment_counter!(state.metrics, register);

    info!("Register payload: {:?}", body);

    Ok(Response::default())
}
