use {
    super::register::RegisterPayload,
    crate::{
        auth::{jwt, AuthBearer},
        error,
        state::AppState,
    },
    axum::{extract::State, Json},
    std::sync::Arc,
};

pub async fn handler(
    State(state): State<Arc<AppState>>,
    AuthBearer(token): AuthBearer,
) -> Result<Json<RegisterPayload>, error::Error> {
    let client_id = jwt::Jwt(token).decode(&state.auth_aud.clone())?;

    let registration = state
        .registration_store
        .get_registration(client_id.value())
        .await?;

    Ok(Json(RegisterPayload {
        tags: registration.tags,
        relay_url: registration.relay_url,
    }))
}
