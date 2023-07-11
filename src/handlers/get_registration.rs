use {
    super::register::RegisterPayload,
    crate::{
        auth::AuthBearer,
        error,
        increment_counter,
        state::{AppState, CachedRegistration},
    },
    axum::{extract::State, Json},
    relay_rpc::{
        domain::ClientId,
        jwt::{JwtBasicClaims, VerifyableClaims},
    },
    std::sync::Arc,
};

pub async fn handler(
    State(state): State<Arc<AppState>>,
    AuthBearer(token): AuthBearer,
) -> Result<Json<RegisterPayload>, error::Error> {
    let claims = JwtBasicClaims::try_from_str(&token)?;
    claims.verify_basic(&state.auth_aud, None)?;
    let client_id = ClientId::from(claims.iss);

    increment_counter!(state.metrics, registration_cache_invalidation);
    state
        .registration_cache
        .invalidate(client_id.as_ref())
        .await;

    let registration = state
        .registration_store
        .get_registration(client_id.as_ref())
        .await?;

    state
        .registration_cache
        .insert(client_id.into_value(), CachedRegistration {
            tags: registration.tags.clone(),
            relay_url: registration.relay_url.clone(),
        })
        .await;

    Ok(Json(RegisterPayload {
        tags: Some(registration.tags),
        append_tags: None,
        remove_tags: None,
        relay_url: registration.relay_url,
    }))
}
