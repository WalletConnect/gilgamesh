use {
    crate::{
        error,
        increment_counter,
        increment_counter_with,
        state::AppState,
        store::messages::{Message, StoreMessages, MAGIC_SKIP_SERIALIZING_METHOD},
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
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Direction {
    Forward,
    Backward,
}

/// The max number of messages to return in the response.
#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMessagesBody {
    pub topic: Arc<str>,
    pub origin_id: Option<Arc<str>>,
    #[serde(default)]
    pub message_count: MessageCount,
    pub direction: Option<Direction>,
}

/////////////////////////

/// The response body for the get messages endpoint.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMessagesResponse {
    pub topic: Arc<str>,
    pub direction: Direction,
    pub next_id: Option<Arc<str>>,
    pub messages: Vec<Message>,
}

/////////////////////////

/// The handler for the get messages endpoint.
pub async fn handler(
    State(state): State<Arc<AppState>>,
    query: Query<GetMessagesBody>,
) -> Result<Json<GetMessagesResponse>, error::Error> {
    let direction = query.direction.unwrap_or(Direction::Forward);

    let StoreMessages { messages, next_id } = match (&query.origin_id, direction) {
        (origin_id, Direction::Forward) => {
            state
                .messages_store
                .get_messages_after(
                    query.topic.as_ref(),
                    origin_id.as_deref(),
                    query.message_count.limit(),
                )
                .await?
        }
        (origin_id, Direction::Backward) => {
            state
                .messages_store
                .get_messages_before(
                    query.topic.as_ref(),
                    origin_id.as_deref(),
                    query.message_count.limit(),
                )
                .await?
        }
    };

    increment_counter!(state.metrics, get_queries);
    increment_counter_with!(state.metrics, served_items, messages.len() as u64);

    // Prematurely make breaking change of removing method
    // https://walletconnect.slack.com/archives/C04CKNV4GN8/p1688127376863359
    let mut messages = messages;
    for message in messages.iter_mut() {
        message.method = MAGIC_SKIP_SERIALIZING_METHOD.to_owned().into();
    }

    let response = GetMessagesResponse {
        topic: query.topic.clone(),
        direction,
        next_id,
        messages,
    };

    Ok(Json(response))
}
