use {
    crate::{
        auth::{jwt, AuthBearer},
        error,
        handlers::Response,
        increment_counter,
        log::prelude::*,
        state::{AppState, CachedRegistration},
    },
    axum::{extract::State, Json},
    serde::{Deserialize, Serialize},
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
    let client_id = jwt::Jwt(token).decode(&state.auth_aud.clone())?;

    state
        .registration_store
        .upsert_registration(
            client_id.value(),
            body.tags.iter().map(AsRef::as_ref).collect(),
            body.relay_url.as_str(),
        )
        .await?;

    state
        .registration_cache
        .insert(client_id.value().to_string(), CachedRegistration {
            tags: body.tags.clone(),
            relay_url: body.relay_url.clone(),
        })
        .await;

    increment_counter!(state.metrics, register);

    Ok(Response::default())
}
