use {
    crate::{
        error,
        handlers::Response,
        increment_counter,
        relay::signature::RequireValidSignature,
        state::{AppState, CachedRegistration},
        store::registrations::Registration,
        tags::match_tag,
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
    RequireValidSignature(Json(payload)): RequireValidSignature<Json<HistoryPayload>>,
) -> error::Result<Response> {
    increment_counter!(state.metrics, received_items);

    let registration = if let Some(registration) = state
        .registration_cache
        .get(payload.client_id.clone().as_str())
        .map(|r| Registration {
            id: None,
            client_id: payload.client_id.clone(),
            tags: r.tags,
            relay_url: r.relay_url,
        }) {
        increment_counter!(state.metrics, cached_registrations);
        registration
    } else {
        let registration = state
            .registration_store
            .get_registration(payload.client_id.as_str())
            .await?;

        state
            .registration_cache
            .insert(payload.client_id.clone(), CachedRegistration {
                tags: registration.tags.clone(),
                relay_url: registration.relay_url.clone(),
            })
            .await;

        increment_counter!(state.metrics, fetched_registrations);
        registration
    };

    let tags = registration.tags;
    for tag in &tags {
        if match_tag(payload.tag, tag) {
            state
                .messages_store
                .upsert_message(
                    payload.client_id.as_str(),
                    payload.topic.as_str(),
                    payload.message_id.as_str(),
                    payload.message.as_str(),
                )
                .await?;

            increment_counter!(state.metrics, stored_items);

            return Ok(Response::default());
        }
    }

    Ok(Response::default())
}
