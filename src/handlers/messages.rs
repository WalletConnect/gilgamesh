use {
    super::Response,
    crate::{
        error::{self},
        state::AppState,
        stores::messages::MongoMessages,
    },
    axum::{
        extract::{Json, Query, State as StateExtractor},
        http::StatusCode,
    },
    serde::{Deserialize, Serialize},
    serde_json::{json, Value},
    std::{cmp, sync::Arc},
};

const MAX_MESSAGE_COUNT: u32 = 500;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostMessageBody {
    pub topic: String,
    pub message_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Direction {
    Forward,
    Backward,
}

#[derive(Deserialize, Debug)]
pub struct MessageCount(u32);

impl Default for MessageCount {
    fn default() -> Self {
        MessageCount(200)
    }
}

impl MessageCount {
    pub fn limit(&self) -> u32 {
        cmp::min(self.0, MAX_MESSAGE_COUNT)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMessagesBody {
    pub topic: String,
    pub origin_id: Option<String>,
    #[serde(default)]
    pub message_count: MessageCount,
    pub direction: Option<Direction>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMessagesResponse {
    pub topic: String,
    pub direction: Direction,
    pub next_id: Option<String>,
    pub messages: Vec<MongoMessages>,
}

impl From<GetMessagesResponse> for Value {
    fn from(response: GetMessagesResponse) -> Self {
        json!(response)
    }
}

pub async fn get(
    StateExtractor(state): StateExtractor<Arc<AppState>>,
    query: Query<GetMessagesBody>,
) -> error::Result<Response> {
    let direction = query.direction.unwrap_or(Direction::Forward);
    let topic = query.topic.clone();

    let (messages, next_id) = match (&query.origin_id, direction) {
        (origin_id, Direction::Forward) => {
            state
                .persistent_storage
                .get_messages_after(
                    topic.as_str(),
                    origin_id.as_deref(),
                    query.message_count.limit(),
                )
                .await?
        }
        (origin_id, Direction::Backward) => {
            state
                .persistent_storage
                .get_messages_before(
                    query.topic.as_str(),
                    origin_id.as_deref(),
                    query.message_count.limit(),
                )
                .await?
        }
    };

    let response = GetMessagesResponse {
        topic,
        direction,
        next_id,
        messages,
    };
    Ok(Response::new_success_with_value(
        StatusCode::OK,
        response.into(),
    ))
}

pub async fn post(
    StateExtractor(state): StateExtractor<Arc<AppState>>,
    body: Json<PostMessageBody>,
) -> error::Result<Response> {
    state
        .persistent_storage
        .upsert_message(body.message_id.as_str(), body.topic.as_str())
        .await?;
    Ok(Response::default())
}
