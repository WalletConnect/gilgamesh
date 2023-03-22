use {
    crate::{
        auth::{jwt, AuthBearer},
        error,
        increment_counter,
        increment_counter_with,
        state::AppState,
        store::messages::{Message, StoreMessages},
    },
    axum::{
        extract::{Query, State},
        Json,
    },
    serde::{Deserialize, Serialize},
    std::{cmp, sync::Arc},
};

/// The absolute max number of messages to return in the response.
pub const MAX_MESSAGE_COUNT: usize = 500;

/////////////////////////

/// The direction to return messages in.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Direction {
    Forward,
    Backward,
}

/// The max number of messages to return in the response.
#[derive(Deserialize, Debug)]
pub struct MessageCount(usize);

impl Default for MessageCount {
    fn default() -> Self {
        MessageCount(200)
    }
}

impl MessageCount {
    pub fn limit(&self) -> usize {
        cmp::min(self.0, MAX_MESSAGE_COUNT)
    }
}

/// The request body for the get messages endpoint.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMessagesBody {
    pub topic: String,
    pub origin_id: Option<String>,
    #[serde(default)]
    pub message_count: MessageCount,
    pub direction: Option<Direction>,
}

/////////////////////////

/// The response body for the get messages endpoint.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMessagesResponse {
    pub topic: String,
    pub direction: Direction,
    pub next_id: Option<String>,
    pub messages: Vec<Message>,
}

/////////////////////////

/// The handler for the get messages endpoint.
pub async fn handler(
    State(state): State<Arc<AppState>>,
    AuthBearer(token): AuthBearer,
    query: Query<GetMessagesBody>,
) -> Result<Json<GetMessagesResponse>, error::Error> {
    let client_id = jwt::Jwt(token).decode(&state.auth_aud.clone())?;

    let direction = query.direction.unwrap_or(Direction::Forward);
    let topic = query.topic.clone();

    let StoreMessages { messages, next_id } = match (&query.origin_id, direction) {
        (origin_id, Direction::Forward) => {
            state
                .messages_store
                .get_messages_after(
                    client_id.value(),
                    topic.as_str(),
                    origin_id.as_deref(),
                    query.message_count.limit(),
                )
                .await?
        }
        (origin_id, Direction::Backward) => {
            state
                .messages_store
                .get_messages_before(
                    client_id.value(),
                    query.topic.as_str(),
                    origin_id.as_deref(),
                    query.message_count.limit(),
                )
                .await?
        }
    };

    increment_counter!(state.metrics, get_queries);
    increment_counter_with!(state.metrics, served_items, messages.len() as u64);

    let response = GetMessagesResponse {
        topic,
        direction,
        next_id,
        messages,
    };

    Ok(Json(response))
}
