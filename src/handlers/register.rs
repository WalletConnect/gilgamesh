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
    relay_rpc::{
        domain::ClientId,
        jwt::{JwtBasicClaims, VerifyableClaims},
    },
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
    let claims = JwtBasicClaims::try_from_str(&token)?;
    claims.verify_basic(&state.auth_aud, None)?;
    let client_id = ClientId::from(claims.iss);

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
