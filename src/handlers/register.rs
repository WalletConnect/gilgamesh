use {
    crate::{
        auth::AuthBearer,
        error::{self, Error},
        handlers::Response,
        increment_counter,
        log::prelude::*,
        state::{AppState, CachedRegistration},
    },
    axum::{extract::State, Json},
    relay_rpc::{
        domain::ClientId,
        jwt::{JwtBasicClaims, VerifyableClaims},
    },
    serde::{Deserialize, Serialize},
    std::{collections::HashSet, sync::Arc},
};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RegisterPayload {
    pub tags: Option<Vec<Arc<str>>>,
    pub append_tags: Option<Vec<Arc<str>>>,
    pub remove_tags: Option<Vec<Arc<str>>>,
    pub relay_url: Arc<str>,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    AuthBearer(token): AuthBearer,
    Json(body): Json<RegisterPayload>,
) -> error::Result<Response> {
    let claims = JwtBasicClaims::try_from_str(&token)?;
    claims.verify_basic(&state.auth_aud, None)?;
    let client_id = ClientId::from(claims.iss);

    increment_counter!(state.metrics, register);

    if let Some(tags) = body.tags {
        increment_counter!(state.metrics, registration_overwrite);

        let tags = tags.into_iter().collect::<HashSet<_>>();
        overwrite_registration(&state, client_id.clone(), tags, body.relay_url).await?;
    } else {
        increment_counter!(state.metrics, registration_update);

        let append_tags = body
            .append_tags
            .map(|tags| tags.into_iter().collect::<HashSet<_>>());
        let remove_tags = body
            .remove_tags
            .map(|tags| tags.into_iter().collect::<HashSet<_>>());

        update_registration(
            &state,
            client_id.clone(),
            append_tags,
            remove_tags,
            body.relay_url,
        )
        .await?;
    }

    Ok(Response::default())
}

async fn overwrite_registration(
    state: &Arc<AppState>,
    client_id: ClientId,
    tags: HashSet<Arc<str>>,
    relay_url: Arc<str>,
) -> error::Result<Response> {
    state
        .registration_store
        .upsert_registration(
            client_id.value(),
            tags.iter().map(AsRef::as_ref).collect(),
            relay_url.as_ref(),
        )
        .await?;

    state
        .registration_cache
        .insert(client_id.into_value(), CachedRegistration {
            tags: tags.into_iter().collect::<Vec<_>>(),
            relay_url,
        })
        .await;

    Ok(Response::default())
}

async fn update_registration(
    state: &Arc<AppState>,
    client_id: ClientId,
    append_tags: Option<HashSet<Arc<str>>>,
    remove_tags: Option<HashSet<Arc<str>>>,
    relay_url: Arc<str>,
) -> error::Result<Response> {
    let append_tags = append_tags.unwrap_or_default();
    let remove_tags = remove_tags.unwrap_or_default();

    if remove_tags.intersection(&append_tags).count() > 0 {
        return Err(Error::InvalidUpdateRequest);
    }

    let registration = state
        .registration_store
        .get_registration(client_id.as_ref())
        .await?;

    let tags = registration
        .tags
        .into_iter()
        .collect::<HashSet<_>>()
        .difference(&remove_tags)
        .cloned()
        .collect::<HashSet<_>>()
        .union(&append_tags)
        .cloned()
        .collect();

    overwrite_registration(state, client_id, tags, relay_url).await
}
