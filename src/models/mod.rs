pub mod user;
pub mod auth;

pub use self::{user::*, auth::*};

use actix_web::{Error, HttpRequest, HttpResponse, Responder, http::StatusCode};
use futures::future::{Ready, ok};
use serde::Serialize;

/// Standard message response
#[derive(Serialize)]
pub struct MessageResponse {
    #[serde(skip_serializing)]
    code: StatusCode,

    message: String,
}

impl MessageResponse {
    /// Create new message response
    pub fn new(code: StatusCode, message: &str) -> Self {
        MessageResponse {
            code: code,
            message: message.to_string(),
        }
    }
    /// New internal server error response
    pub fn internal_server_error() -> Self {
        MessageResponse::new(StatusCode::INTERNAL_SERVER_ERROR, "There was a problem processing your request")
    }
    /// Create new unauthorized error response
    pub fn unauthorized_error() -> Self {
        MessageResponse::new(StatusCode::UNAUTHORIZED, "You are not authorized to make this request")
    }
    /// Create new bad request error response
    pub fn bad_request() -> Self {
        MessageResponse::new(StatusCode::BAD_REQUEST, "You sent an invalid request")
    }
    /// Explicit convert to actix HttpResponse type
    pub fn http_response(&self) -> HttpResponse {
        HttpResponse::build(self.code)
            .json(self)
    }
}

/// Implicit From convert to actix HttpResponse type
impl From<MessageResponse> for HttpResponse {
    fn from(response: MessageResponse) -> Self {
        response.http_response()
    }
}

/// Convert to actix Error type
impl From<MessageResponse> for Error {
    fn from(response: MessageResponse) -> Self {
        HttpResponse::from(response).into()
    }
}

/// Responder to convert data to valid simple HTTP response
impl Responder for MessageResponse {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    /// Get HTTP response from response
    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        ok(HttpResponse::from(self))
    }
}