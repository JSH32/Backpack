use sea_orm::DatabaseConnection;

use crate::database::entity::albums;
use std::sync::Arc;

use super::prelude::*;

pub struct AlbumService {
    database: Arc<DatabaseConnection>,
}

data_service_owned!(AlbumService, albums);

impl AlbumService {
    pub fn new(database: Arc<DatabaseConnection>) -> Self {
        Self { database }
    }
}
