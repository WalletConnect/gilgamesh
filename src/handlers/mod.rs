use {
    axum::{response::IntoResponse, Json},
    hyper::StatusCode,
    serde_json::{json, Value},
};

pub mod health;
pub mod messages;

#[derive(serde::Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ResponseStatus {
    Success,
    Failure,
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
    pub error: Option<ResponseError>,
    pub value: Option<Value>,
}

impl Response {
    pub fn new_success_with_value(status: StatusCode, value: Value) -> Self {
        Response {
            status: ResponseStatus::Success,
            status_code: status,
            error: None,
            value: Some(value),
        }
    }

    pub fn new_success(status: StatusCode) -> Self {
        Response {
            status: ResponseStatus::Success,
            status_code: status,
            error: None,
            value: None,
        }
    }

    pub fn new_failure(status: StatusCode, error: ResponseError) -> Self {
        Response {
            status: ResponseStatus::Failure,
            status_code: status,
            error: Some(error),
            value: None,
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
    fn from(response: Response) -> Self {
        Json(json!(response))
    }
}

impl Default for Response {
    fn default() -> Self {
        Response::new_success(StatusCode::OK)
    }
}
