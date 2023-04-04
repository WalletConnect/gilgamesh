use {
    crate::{
        error,
        handlers::Response,
        increment_counter,
        log::prelude::*,
        relay::signature::RequireValidSignature,
        state::{AppState, CachedRegistration},
        store::{registrations::Registration, StoreError},
        tags::match_tag,
    },
    axum::{extract::State as StateExtractor, Json},
    serde::{Deserialize, Serialize},
    std::sync::Arc,
};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HistoryPayload {
    pub method: Arc<str>,
    pub client_id: Arc<str>,
    pub topic: Arc<str>,
    pub message_id: Arc<str>,
    pub tag: u32,
    pub message: Arc<str>,
}

pub async fn handler(
    StateExtractor(state): StateExtractor<Arc<AppState>>,
    RequireValidSignature(Json(payload)): RequireValidSignature<Json<HistoryPayload>>,
) -> error::Result<Response> {
    debug!("Received `save_message` query: {:?}", payload);

    increment_counter!(state.metrics, received_items);

    let registration = if let Some(registration) = state
        .registration_cache
        .get(payload.client_id.as_ref())
        .map(|r| Registration {
            id: None,
            client_id: payload.client_id.clone(),
            tags: r.tags,
            relay_url: r.relay_url,
        }) {
        debug!("loaded registration from cache");
        increment_counter!(state.metrics, cached_registrations);
        registration
    } else {
        debug!("loading registration from database");
        let registration = match state
            .registration_store
            .get_registration(payload.client_id.as_ref())
            .await
        {
            Ok(registration) => registration,
            Err(StoreError::NotFound(_, _)) => return Ok(Response::default()),
            Err(e) => return Err(e.into()),
        };

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
            debug!("tag matching, storing message");
            state
                .messages_store
                .upsert_message(
                    payload.method.as_ref(),
                    payload.client_id.as_ref(),
                    payload.topic.as_ref(),
                    payload.message_id.as_ref(),
                    payload.message.as_ref(),
                )
                .await?;

            debug!("message stored, sending ack");

            increment_counter!(state.metrics, stored_items);

            return Ok(Response::default());
        }
    }

    Ok(Response::default())
}
