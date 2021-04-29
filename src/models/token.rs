use serde::{Serialize, Deserialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenData {
    pub id: i32,
    pub name: String,
    
    #[serde(skip_serializing)]
    pub user_id: i32,

    // Only send token when the token is originally created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Deserialize)]
pub struct TokenCreateForm {
    pub name: String
}

#[derive(Deserialize)]
pub struct TokenQuery {
    pub id: i32
}