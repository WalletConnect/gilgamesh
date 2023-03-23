use {
    super::register::RegisterPayload,
    crate::{
        auth::AuthBearer,
        error,
        increment_counter,
        state::{AppState, CachedRegistration},
    },
    axum::{extract::State, Json},
    relay_rpc::auth::Jwt,
    std::sync::Arc,
};

pub async fn handler(
    State(state): State<Arc<AppState>>,
    AuthBearer(token): AuthBearer,
) -> Result<Json<RegisterPayload>, error::Error> {
    let client_id = Jwt(token).decode(&state.auth_aud.clone())?.to_string();

    increment_counter!(state.metrics, registration_cache_invalidation);
    state
        .registration_cache
        .invalidate(client_id.as_str())
        .await;

    let registration = state
        .registration_store
        .get_registration(client_id.as_str())
        .await?;

    state
        .registration_cache
        .insert(client_id, CachedRegistration {
            tags: registration.tags.clone(),
            relay_url: registration.relay_url.clone(),
        })
        .await;

    Ok(Json(RegisterPayload {
        tags: registration.tags,
        relay_url: registration.relay_url,
    }))
}
