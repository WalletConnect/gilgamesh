use {
    crate::state::AppState,
    axum::{
        extract::{Json, State as StateExtractor},
        http::StatusCode,
        response::IntoResponse,
    },
    serde::{Deserialize, Serialize},
    std::sync::Arc,
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostMessagesBody {}

pub async fn get(StateExtractor(state): StateExtractor<Arc<AppState>>) -> impl IntoResponse {
    (StatusCode::OK, "OK".to_string())
}

pub async fn post(
    StateExtractor(state): StateExtractor<Arc<AppState>>,
    body: Json<PostMessagesBody>,
) -> impl IntoResponse {
    (StatusCode::OK, "OK".to_string())
}
