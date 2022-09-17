use crate::models::{MessageResponse, Page};
use actix_http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

pub mod application;
pub mod auth;
pub mod data_service;
pub mod file;
pub mod registration_key;
pub mod user;

pub mod prelude {
    pub use super::{
        ServiceError, ServicePage, ServiceResult, ToMessageResponse, ToPageResponse, ToResponse,
    };
    pub use crate::services::data_service::*;
}

pub type ServiceResult<T> = Result<T, ServiceError>;

pub struct ServicePage<T> {
    pub page: usize,
    pub pages: usize,
    pub items: Vec<T>,
}

pub trait ToMessageResponse {
    /// Converts a [`ServiceResult<String>`] to a HTTP response of [`MessageResponse`].
    fn to_message_response(self, code: StatusCode) -> HttpResponse;
}

impl ToMessageResponse for ServiceResult<String> {
    fn to_message_response(self, code: StatusCode) -> HttpResponse {
        match self {
            Ok(v) => MessageResponse::new(code, &v).http_response(),
            Err(e) => e.to_response(),
        }
    }
}

/// Converts a [`ServiceResult`] to an HTTP response.
pub trait ToResponse<T> {
    /// Converts a [`ServiceResult`] to an HTTP response of type [`R`]
    fn to_response<R: From<T> + Serialize>(self, code: StatusCode) -> HttpResponse;
}

impl<T> ToResponse<T> for ServiceResult<T> {
    fn to_response<R: From<T> + Serialize>(self, code: StatusCode) -> HttpResponse {
        match self {
            Ok(v) => HttpResponse::build(code).json(R::from(v)),
            Err(e) => e.to_response(),
        }
    }
}

/// Converts a [`ServiceResult<ServicePage<T>>`] to an HTTP response.
/// NOTE: Using [`ToResponse`] causes conflicts with the existing implementation.
pub trait ToPageResponse<T> {
    /// Converts a [`ServiceResult<ServicePage<T>>`] to an HTTP response of type [`Page<R>`].
    fn to_page_response<R: From<T> + Serialize>(self, code: StatusCode) -> HttpResponse;
}

impl<T> ToPageResponse<T> for ServiceResult<ServicePage<T>> {
    fn to_page_response<R: From<T> + Serialize>(self, code: StatusCode) -> HttpResponse {
        match self {
            Ok(v) => HttpResponse::build(code).json(Page {
                page: v.page,
                pages: v.pages,
                items: v.items.into_iter().map(|item| R::from(item)).collect(),
            }),
            Err(e) => e.to_response(),
        }
    }
}

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Database error: {0}")]
    DbErr(sea_orm::DbErr),
    #[error("Server error: {0}")]
    ServerError(anyhow::Error),
    #[error("{0} was not found")]
    NotFound(String),
    #[error("{0}")]
    InvalidData(String),
    #[error("{0}")]
    Conflict(String),
    #[error("{0}")]
    TooLarge(String),
    #[error("{0}")]
    Unauthorized(String),
    #[error("You are not allowed to access this {resource}")]
    Forbidden { id: String, resource: String },
}

impl ServiceError {
    pub fn to_response(&self) -> HttpResponse {
        let message = self.to_string();

        MessageResponse::new(
            match self.http_code() {
                StatusCode::INTERNAL_SERVER_ERROR => {
                    // This is worth logging.
                    log::error!("{}", message);
                    StatusCode::INTERNAL_SERVER_ERROR
                }
                v => v,
            },
            &message,
        )
        .http_response()
    }

    pub fn http_code(&self) -> StatusCode {
        match self {
            Self::DbErr(_) | Self::ServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::InvalidData(_) => StatusCode::BAD_REQUEST,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::TooLarge(_) => StatusCode::PAYLOAD_TOO_LARGE,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Forbidden { id: _, resource: _ } => StatusCode::FORBIDDEN,
        }
    }

    /// Create a simple unauthorized error.
    pub fn unauthorized() -> ServiceError {
        ServiceError::Unauthorized("You are not authorized to make this request".into())
    }
}

impl ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        self.http_code()
    }

    fn error_response(&self) -> HttpResponse {
        self.to_response()
    }
}
