pub mod application;
pub mod auth;
pub mod file;
pub mod user;
pub mod registration_key;

use core::fmt;
use std::fmt::{Debug, Display};

use actix_web::{http::StatusCode, HttpRequest, HttpResponse, Responder, ResponseError};
use derive_more::Display;
use sea_orm::ActiveEnum;
use serde::Serialize;
use thiserror::Error;

use crate::database::entity::settings;

pub use self::{application::*, auth::*, file::*, user::*};

#[derive(Debug, Display)]
pub struct Error(anyhow::Error);

/// # Response
///
/// Utility type for error reporting.
///
/// The error variant accepts any error as it wraps [`anyhow::Error`].
/// This type should be returned from an Actix route handler.
/// Error variant should only be used when returning an exceptional case.
///
/// # Usage
/// ```
/// fn route() -> Response<()> {
///     Err("This could be any error type")
/// }
/// ```
pub type Response<T> = Result<T, Error>;

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        log::error!("{}", &self.0.to_string());
        MessageResponse::internal_server_error(&self.0.to_string()).http_response()
    }
}

impl<E: Into<anyhow::Error>> From<E> for Error {
    fn from(e: E) -> Self {
        Self(e.into())
    }
}

#[derive(Error, Debug)]
#[error("{0}")]
pub struct StringError(pub String);

/// Standard message response
#[derive(Serialize, Debug)]
pub struct MessageResponse {
    #[serde(skip_serializing)]
    code: StatusCode,

    message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,

    // Optional data, can be any JSON value
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
}

impl MessageResponse {
    /// Create new message response
    pub fn new(code: StatusCode, message: &str) -> Self {
        MessageResponse {
            code: code,
            message: message.to_string(),
            data: None,
            error: None,
        }
    }

    pub fn new_with_data(code: StatusCode, message: &str, data: serde_json::Value) -> Self {
        MessageResponse {
            code: code,
            message: message.to_string(),
            data: Some(data),
            error: None,
        }
    }

    /// New internal server error response
    pub fn internal_server_error(error: &str) -> Self {
        let mut response = MessageResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "There was a problem processing your request",
        );

        response.error = Some(error.to_string());

        response
    }

    /// Create new unauthorized error response
    pub fn unauthorized_error() -> Self {
        MessageResponse::new(
            StatusCode::UNAUTHORIZED,
            "You are not authorized to make this request",
        )
    }

    /// Create new bad request error response
    pub fn bad_request() -> Self {
        MessageResponse::new(StatusCode::BAD_REQUEST, "You sent an invalid request")
    }

    /// Explicit convert to actix HttpResponse type
    pub fn http_response(&self) -> HttpResponse {
        HttpResponse::build(self.code).json(self)
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

#[derive(Serialize)]
pub struct Page<T> {
    pub page: usize,
    pub pages: usize,
    pub list: Vec<T>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    pub app_name: String,
    pub app_description: String,
    pub color: String,
    pub version: Option<String>,
    pub commit: Option<String>,
}

impl From<settings::Model> for AppInfo {
    fn from(settings: settings::Model) -> Self {
        Self {
            app_name: settings.app_name,
            app_description: settings.app_description,
            color: settings.color.to_value(),
            version: None,
            commit: None,
        }
    }
}

impl AppInfo {
    pub fn set_commit(&mut self, commit: String) {
       self.commit = Some(commit);
    }
    // pub fn set_version(&mut self, version: String) {
    //     self.version = Some(version);
    // }
}