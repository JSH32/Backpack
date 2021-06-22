use crate::{database::Database, storage::StorageProvider};

pub struct State {
    pub database: Database,
    pub storage: Box<dyn StorageProvider>,
    pub jwt_key: String,
}