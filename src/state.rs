use crate::{database::Database, storage::Storage};

pub struct State {
    pub database: Database,
    pub storage: Storage
}