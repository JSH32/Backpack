use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams)]
pub struct FileQuery {
    /// Filename search
    pub search: Option<String>,
    /// File uploader ID
    pub user: Option<String>,
}
