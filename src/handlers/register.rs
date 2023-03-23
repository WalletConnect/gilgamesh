use {
    crate::{
        auth::AuthBearer,
        error,
        handlers::Response,
        increment_counter,
        log::prelude::*,
        state::{AppState, CachedRegistration},
    },
    axum::{extract::State, Json},
    relay_rpc::auth::Jwt,
    serde::{Deserialize, Serialize},
    std::sync::Arc,
};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RegisterPayload {
    pub tags: Vec<Arc<str>>,
    pub relay_url: Arc<str>,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    AuthBearer(token): AuthBearer,
    Json(body): Json<RegisterPayload>,
) -> error::Result<Response> {
    let client_id = Jwt(token).decode(&state.auth_aud.clone())?;

    state
        .registration_store
        .upsert_registration(
            client_id.value(),
            body.tags.iter().map(AsRef::as_ref).collect(),
            body.relay_url.as_ref(),
        )
        .await?;

    state
        .registration_cache
        .insert(client_id.into_value(), CachedRegistration {
            tags: body.tags.clone(),
            relay_url: body.relay_url.clone(),
        })
        .await;

    increment_counter!(state.metrics, register);

    Ok(Response::default())
}
