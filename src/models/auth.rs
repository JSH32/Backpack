use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct BasicAuthForm {
    pub auth: String,
    pub password: String,
}

/// OAuth redirect request parameters.
#[derive(Deserialize, ToSchema)]
pub struct OAuthRequest {
    pub code: String,
    pub state: String,
}

/// Enabled authorization methods.
#[derive(Serialize, ToSchema, Default)]
pub struct AuthMethods {
    /// Is password authentication enabled.
    pub password: bool,
    /// Google username (email before the @).
    pub google: Option<String>,
    /// Cached github username.
    pub github: Option<String>,
    /// Cached discord tag.
    pub discord: Option<String>,
}
