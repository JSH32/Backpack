use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct BasicAuthForm {
    pub auth: String,
    pub password: String,
}

/// OAuth redirect request parameters.
#[derive(Deserialize, ToSchema)]
pub struct AuthRequest {
    pub code: String,
    pub state: String,
}
