use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct TokenData {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,

    /// May be none because of listing and less important data, do not always want to expose token unless explicitly requested
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    #[serde(skip_serializing)]
    pub user_id: i32
}

#[derive(Deserialize)]
pub struct TokenCreateForm {
    pub description: Option<String>,
    pub name: String
}