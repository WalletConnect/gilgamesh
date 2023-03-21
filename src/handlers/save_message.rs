use {
    crate::{
        error,
        handlers::Response,
        increment_counter,
        relay::signature::RequireValidSignature,
        state::AppState,
    },
    axum::{extract::State as StateExtractor, Json},
    serde::{Deserialize, Serialize},
    std::sync::Arc,
};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HistoryPayload {
    pub client_id: String,
    pub topic: String,
    pub message_id: String,
    pub tag: u32,
    pub message: String,
}

pub async fn handler(
    StateExtractor(state): StateExtractor<Arc<AppState>>,
    RequireValidSignature(Json(body)): RequireValidSignature<Json<HistoryPayload>>,
) -> error::Result<Response> {
    increment_counter!(state.metrics, received_items);

    state
        .messages_store
        .upsert_message(body.topic.as_str(), body.message_id.as_str())
        .await?;

    increment_counter!(state.metrics, stored_items);

    Ok(Response::default())
}
