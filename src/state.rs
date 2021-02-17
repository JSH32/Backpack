use crate::{database::Database, storage::Storage};
use hmac::Hmac;
use sha2::Sha256;

pub struct State {
    pub database: Database,
    pub storage: Storage,
    pub jwt_key: Hmac<Sha256>
}