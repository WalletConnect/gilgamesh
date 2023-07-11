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
    std::sync::Arc,
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
        overwrite_registration(&state, client_id.clone(), tags, body.relay_url).await?;
    } else {
        increment_counter!(state.metrics, registration_update);
        update_registration(
            &state,
            client_id.clone(),
            body.append_tags,
            body.remove_tags,
            body.relay_url,
        )
        .await?;
    }

    Ok(Response::default())
}

async fn overwrite_registration(
    state: &Arc<AppState>,
    client_id: ClientId,
    tags: Vec<Arc<str>>,
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
            tags,
            relay_url,
        })
        .await;

    Ok(Response::default())
}

async fn update_registration(
    state: &Arc<AppState>,
    client_id: ClientId,
    append_tags: Option<Vec<Arc<str>>>,
    remove_tags: Option<Vec<Arc<str>>>,
    relay_url: Arc<str>,
) -> error::Result<Response> {
    if let (Some(append_tags), Some(remove_tags)) = (append_tags.clone(), remove_tags.clone()) {
        for tag in remove_tags.iter() {
            if append_tags.contains(tag) {
                return Err(Error::InvalidUpdateRequest);
            }
        }
    }

    let registration = state
        .registration_store
        .get_registration(client_id.as_ref())
        .await?;

    let mut tags = registration.tags;

    if let Some(remove_tags) = remove_tags {
        for r_tag in remove_tags.iter() {
            if let Some(index) = tags.iter().position(|tag| tag.eq(r_tag)) {
                tags.remove(index);
            }
        }
    }

    if let Some(append_tags) = append_tags {
        for a_tag in append_tags.iter() {
            if !tags.contains(a_tag) {
                tags.push(a_tag.clone());
            }
        }
    }

    overwrite_registration(state, client_id, tags, relay_url).await
}
