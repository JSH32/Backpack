pub mod user;
pub mod auth;
pub mod file;
pub mod application;

use core::fmt;
use std::fmt::{Debug, Display};

pub use self::{user::*, auth::*};

use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError, http::StatusCode};
use serde::{Serialize, Deserialize};

/// Query for any data with an ID
#[derive(Deserialize)]
pub struct IDQuery {
    pub id: i32
}

/// Standard message response
#[derive(Serialize, Debug)]
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

impl Display for MessageResponse {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "code: {}, message: {}", self.code, self.message)
    }
}

impl ResponseError for MessageResponse {
    fn status_code(&self) -> StatusCode {
        self.code
    }

    fn error_response(&self) -> HttpResponse {
        self.http_response()
    }
}

/// Responder to convert data to valid simple HTTP response
impl Responder for MessageResponse {
    /// Get HTTP response from response
    fn respond_to(self, _: &HttpRequest) -> HttpResponse {
        HttpResponse::from(self)
    }
}