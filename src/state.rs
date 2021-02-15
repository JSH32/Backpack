use std::sync::{Mutex};

use crate::{database::Database, storage::Storage};

pub struct State {
    pub database: Database,
    pub storage: Storage
}