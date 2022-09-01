use serde::Deserialize;
use utoipa::{Component, IntoParams};

#[derive(Deserialize, IntoParams, Component)]
#[serde(rename_all = "camelCase")]
pub struct FileQuery {
    pub search: Option<String>,
    pub user_id: Option<String>,
}
