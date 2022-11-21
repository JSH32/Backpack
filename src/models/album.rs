use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::database::entity::albums;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AlbumData {
    pub id: String,
    pub name: String,
    /// Optional album description
    pub description: Option<String>,
    /// User who created the album.
    pub user_id: String,
    /// Is the album public.
    pub public: bool,
    /// Date of album creation
    #[schema(value_type = String)]
    pub created: DateTimeUtc,
}

impl From<albums::Model> for AlbumData {
    fn from(album: albums::Model) -> Self {
        Self {
            id: album.id,
            name: album.name,
            description: album.description,
            user_id: album.user_id,
            created: album.created,
            public: album.public,
        }
    }
}

#[derive(Deserialize, IntoParams)]
pub struct AlbumDelete {
    pub delete_files: Option<bool>,
}
