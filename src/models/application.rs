use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationData {
    pub id: String,
    pub name: String,

    #[serde(skip_serializing)]
    pub user_id: String,

    // Only send token when the token is originally created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
}

#[derive(Deserialize)]
pub struct ApplicationCreateForm {
    pub name: String,
}
