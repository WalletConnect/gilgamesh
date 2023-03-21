use {
    super::register::RegisterPayload,
    crate::{
        auth::{jwt, AuthBearer},
        error,
        increment_counter,
        state::{AppState, CachedRegistration},
    },
    axum::{extract::State, Json},
    std::sync::Arc,
};

pub async fn handler(
    State(state): State<Arc<AppState>>,
    AuthBearer(token): AuthBearer,
) -> Result<Json<RegisterPayload>, error::Error> {
    let client_id = jwt::Jwt(token).decode(&state.auth_aud.clone())?;

    increment_counter!(state.metrics, registration_cache_invalidation);
    state.registration_cache.invalidate(client_id.value()).await;

    let registration = state
        .registration_store
        .get_registration(client_id.value())
        .await?;

    state
        .registration_cache
        .insert(client_id.value().to_string(), CachedRegistration {
            tags: registration.tags.clone(),
            relay_url: registration.relay_url.clone(),
        })
        .await;

    Ok(Json(RegisterPayload {
        tags: registration.tags,
        relay_url: registration.relay_url,
    }))
}
