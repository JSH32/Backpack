use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;

use crate::models::MessageResponse;

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
///     Err(anyhow::anyhow!("This could be any error type"))
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
