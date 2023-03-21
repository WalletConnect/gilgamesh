use {
    axum::{response::IntoResponse, Json},
    hyper::StatusCode,
    serde_json::{json, Value},
};

pub mod get_messages;
pub mod get_registration;
pub mod health;
pub mod metrics;
pub mod register;
pub mod save_message;

#[derive(serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ErrorLocation {
    Body,
    Header,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ResponseStatus {
    Success,
    Failure,
}

#[derive(serde::Serialize)]
pub struct ErrorField {
    pub field: String,
    pub description: String,
    pub location: ErrorLocation,
}

#[derive(serde::Serialize)]
pub struct ResponseError {
    pub name: String,
    pub message: String,
}

#[derive(serde::Serialize)]
pub struct Response {
    pub status: ResponseStatus,
    #[serde(skip_serializing)]
    pub status_code: StatusCode,
    pub errors: Option<Vec<ResponseError>>,
    pub fields: Option<Vec<ErrorField>>,
}
impl Response {
    pub fn new_success(status: StatusCode) -> Self {
        Response {
            status: ResponseStatus::Success,
            status_code: status,
            errors: None,
            fields: None,
        }
    }

    pub fn new_failure(
        status: StatusCode,
        errors: Vec<ResponseError>,
        fields: Vec<ErrorField>,
    ) -> Self {
        Response {
            status: ResponseStatus::Failure,
            status_code: status,
            errors: Some(errors),
            fields: Some(fields),
        }
    }
}

impl IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        let status = self.status_code;
        let json: Json<Value> = self.into();

        (status, json).into_response()
    }
}

impl From<Response> for Json<Value> {
    fn from(value: Response) -> Self {
        Json(json!(value))
    }
}

impl Default for Response {
    fn default() -> Self {
        Response::new_success(StatusCode::OK)
    }
}
