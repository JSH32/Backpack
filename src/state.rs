use lettre::{AsyncSmtpTransport, Tokio1Executor};
use crate::{database::Database, sonyflake::Sonyflake, storage::StorageProvider};

pub struct State {
    pub database: Database,
    pub storage: Box<dyn StorageProvider>,
    pub jwt_key: String,
    pub base_url: String,
    pub smtp_client: Option<(AsyncSmtpTransport<Tokio1Executor>, String)>
}