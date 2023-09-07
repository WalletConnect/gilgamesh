use {
    async_trait::async_trait,
    axum::{
        extract::FromRequestParts,
        http::{header::AUTHORIZATION, request::Parts, StatusCode},
    },
};

const ERR_MISSING: &str = "`Authorization` header is missing";
const ERR_CHARS: &str = "`Authorization` header contains invalid characters";
const ERR_WRONG_BEARER: &str = "`Authorization` header must be a bearer token";

/// Rejection error used in the [AuthBearer] extractors.
pub type Rejection = (StatusCode, &'static str);

/// Bearer token extractor which contains the innards of a bearer header as a
/// string.
///
/// # Example
///
/// This structure can be used like any other axum extractor:
///
/// ```no_run
/// use gilgamesh::auth::AuthBearer;
///
/// /// Handler for a typical [axum] route, takes a `token` and returns it
/// async fn handler(AuthBearer(token): AuthBearer) -> String {
///     format!("Found a bearer token: {}", token)
/// }
/// ```
///
/// # Errors
///
/// There are a few errors which this extractor can make. By default, all
/// invalid responses are `400 BAD REQUEST` with one of these messages:
///
/// - \`Authorization\` header must be a bearer token – Somebody tried to but
///   basic auth here instead of bearer
/// - \`Authorization\` header is missing – The header was required but it
///   wasn't found
/// - \`Authorization\` header contains invalid characters – The header couldn't
///   be processed because of invalid characters
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AuthBearer(pub String);

#[async_trait]
impl<B> FromRequestParts<B> for AuthBearer
where
    B: Send + Sync,
{
    type Rejection = Rejection;

    async fn from_request_parts(req: &mut Parts, _: &B) -> Result<Self, Self::Rejection> {
        Self::decode_request_parts(req)
    }
}

impl AuthBearer {
    fn from_header(contents: &str) -> Self {
        Self(contents.to_string())
    }

    fn decode_request_parts(req: &Parts) -> Result<Self, Rejection> {
        // Get authorization header
        let authorization = req
            .headers
            .get(AUTHORIZATION)
            .ok_or((StatusCode::BAD_REQUEST, ERR_MISSING))?
            .to_str()
            .map_err(|_| (StatusCode::BAD_REQUEST, ERR_CHARS))?;

        // Check that its a well-formed bearer and return
        let split = authorization.split_once(' ');
        match split {
            // Found proper bearer
            Some(("Bearer", contents)) => Ok(Self::from_header(contents)),
            // Found empty bearer
            _ if authorization == "Bearer" => Ok(Self::from_header("")),
            // Found nothing
            _ => Err((StatusCode::BAD_REQUEST, ERR_WRONG_BEARER)),
        }
    }
}
