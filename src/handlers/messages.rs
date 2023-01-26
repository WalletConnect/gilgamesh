use axum::extract::Query;

use {
    super::Response,
    crate::{
        error::{self},
        state::AppState,
        stores::messages::MongoMessages,
    },
    axum::{
        extract::{Json, State as StateExtractor},
        http::StatusCode,
        response::IntoResponse,
    },
    serde::{Deserialize, Serialize},
    serde_json::{json, Value},
    std::sync::Arc,
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostMessagesBody {
    pub message_id: String,
    pub topic: String,
}

#[derive(Deserialize)]
pub struct GetMessagesBody {
    pub topic: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMessagesResponse {
    pub messages: Vec<MongoMessages>,
}

impl From<GetMessagesResponse> for Value {
    fn from(response: GetMessagesResponse) -> Self {
        json!(response)
    }
}

pub async fn get(
    StateExtractor(state): StateExtractor<Arc<AppState>>,
    topic: Query<GetMessagesBody>,
) -> error::Result<Response> {
    let messages: Vec<MongoMessages> = state
        .persistent_storage
        .get_messages(topic.topic.as_str())
        .await?;
    let response = GetMessagesResponse { messages };
    Ok(Response::new_success_with_value(
        StatusCode::OK,
        response.into(),
    ))
}

pub async fn post(
    StateExtractor(state): StateExtractor<Arc<AppState>>,
    body: Json<PostMessagesBody>,
) -> error::Result<Response> {
    state
        .persistent_storage
        .upsert_message(body.message_id.as_str(), body.topic.as_str())
        .await?;
    Ok(Response::default())
}
