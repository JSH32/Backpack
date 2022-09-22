use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::services::auth::oauth::OAuthProvider;

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

impl AuthMethods {
    /// Get the amount of enabled auth methods.
    pub fn enabled_methods(&self) -> u8 {
        return (self.password as u8)
            + (self.google.is_some() as u8)
            + (self.github.is_some() as u8)
            + (self.discord.is_some() as u8);
    }
}

#[derive(Deserialize, IntoParams)]
pub struct OAuthLoginQuery {
    pub provider: OAuthProvider,
    pub redirect: Option<String>,
    pub include_token: bool,
}

#[derive(Serialize, ToSchema)]
pub struct LoginRedirectUrl {
    pub url: String,
}

/// Unlink an OAuth method.
#[derive(Deserialize, ToSchema)]
pub struct UnlinkAuthMethod {
    pub method: OAuthProvider,
    /// Password required if present.
    pub password: Option<String>,
}
