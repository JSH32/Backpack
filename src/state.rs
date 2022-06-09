use actix_http::Uri;
use lettre::{AsyncSmtpTransport, Tokio1Executor};
use sea_orm::DatabaseConnection;

use crate::storage::StorageProvider;

pub struct State {
    pub database: DatabaseConnection,
    pub storage: Box<dyn StorageProvider>,
    pub jwt_key: String,
    pub api_url: Uri,
    pub client_url: Uri,
    pub storage_url: String,
    pub smtp_client: Option<(AsyncSmtpTransport<Tokio1Executor>, String)>,
    pub file_size_limit: usize,
    pub invite_only: bool,
}
