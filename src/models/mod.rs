pub mod user;
pub mod auth;

use actix_web::{Error, HttpRequest, HttpResponse, Responder, http::StatusCode};
use futures::future::{Ready, ok};
use serde_json::Value;
use serde::Serialize;

/// Standard response spec for API
#[derive(Serialize)]
pub struct Response {
    code: u16,
    error: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>
}

impl Response {
    // Create new message response
    pub fn new_message(code: StatusCode, error: bool, message: &str) -> Self {
        Response {
            code: code.as_u16(),
            error: error,
            message: Some(message.to_string()),
            data: None
        }
    }
    // Create new data response
    pub fn new_data(code: StatusCode, error: bool, data: Value) -> Self {
        Response {
            code: code.as_u16(),
            error: error,
            message: None,
            data: Some(data)
        }
    }
    /// Create new internal server error response
    pub fn internal_server_error() -> Self {
        Response::new_message(StatusCode::INTERNAL_SERVER_ERROR, true, "There was a problem processing your request")
    }
}

/// Convert to actix HttpResponse type
impl From<Response> for HttpResponse {
    fn from(response: Response) -> Self {
        HttpResponse::build(StatusCode::from_u16(response.code).unwrap())
            .json(response)
    }
}

/// Convert to actix Error type
impl From<Response> for Error {
    fn from(response: Response) -> Self {
        HttpResponse::from(response).into()
    }
}

/// Responder to convert data to valid simple HTTP response
impl Responder for Response {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    /// Get HTTP response from response
    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        ok(HttpResponse::from(self))
    }
}