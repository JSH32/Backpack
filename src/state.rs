use crate::{database::Database, storage::StorageProvider};
use hmac::Hmac;
use sha2::Sha256;

pub struct State {
    pub database: Database,
    pub storage: Box<dyn StorageProvider + Sync + Send>,
    pub jwt_key: Hmac<Sha256>
}