pub mod admin;
pub mod application;
pub mod auth;
pub mod file;
pub mod user;

use crate::{database::entity::settings, internal::GIT_VERSION};
use actix_http::body::BoxBody;
use actix_web::{http::StatusCode, HttpRequest, HttpResponse, Responder, ResponseError};
use core::fmt;
use sea_orm::ActiveEnum;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Display;
use utoipa::ToSchema;

use self::registration_key::RegistrationKeyData;
pub use self::{admin::*, application::*, auth::*, file::*, user::*};

/// Standard message response.
///
/// Usually the only field will be `message`
#[derive(Serialize, Debug, ToSchema)]
#[schema(example = json!({"message": "string"}))]
pub struct MessageResponse {
    #[serde(skip)]
    code: StatusCode,

    /// Message
    message: String,

    /// Optional error (only on 500 errors)
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,

    /// Optional data, can be any JSON object
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Object)]
    data: Option<HashMap<String, serde_json::Value>>,
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

    pub fn ok<E>(code: StatusCode, message: &str) -> Result<HttpResponse, E> {
        Ok(MessageResponse::new(code, message).http_response())
    }

    pub fn ok_with_data<E>(
        code: StatusCode,
        message: &str,
        data: HashMap<String, serde_json::Value>,
    ) -> Result<HttpResponse, E> {
        Ok(MessageResponse::new_with_data(code, message, data).http_response())
    }

    pub fn new_with_data(
        code: StatusCode,
        message: &str,
        data: HashMap<String, serde_json::Value>,
    ) -> Self {
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
    type Body = BoxBody;

    /// Get HTTP response from response
    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::from(self)
    }
}

#[derive(Serialize, ToSchema)]
#[aliases(
    FilePage = Page<FileData>,
    RegistrationKeyPage = Page<RegistrationKeyData>,
    ApplicationPage = Page<ApplicationData>
)]
pub struct Page<T> {
    pub page: usize,
    pub pages: usize,
    pub items: Vec<T>,
}

/// Public server configuration
#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    /// App name
    pub app_name: String,

    /// App description
    pub app_description: String,

    /// Theme color of the Backpack instance
    pub color: String,

    /// Are registration keys enabled?
    pub invite_only: bool,

    /// Is SMTP (email verification) enabled on the server?
    pub smtp: bool,

    /// Git tag (version) or commit hash
    pub git_version: String,

    /// Amount of files uploaded.
    pub uploaded_files: usize,
}

impl AppInfo {
    pub fn new(
        settings_model: settings::Model,
        invite_only: bool,
        smtp: bool,
        uploaded_files: usize,
    ) -> Self {
        Self {
            app_name: settings_model.app_name,
            app_description: settings_model.app_description,
            color: settings_model.color.to_owned().to_value(),
            invite_only,
            smtp,
            git_version: GIT_VERSION.to_string(),
            uploaded_files,
        }
    }
}
