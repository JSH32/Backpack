use derive_more::Display;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use serde::Deserialize;

use crate::config::OAuthConfig;
use crate::models::AuthRequest;
use crate::services::{ServiceError, ServiceResult};

/// All OAuth providers.
#[derive(Debug, Display)]
pub enum OAuthProvider {
    Google,
    Github,
    Discord,
}

impl OAuthProvider {
    /// Create new client for the provider.
    ///
    /// # Arguments
    ///
    /// * `config` - OAuth config.
    /// * `callback_url` - Callback URL.
    pub fn new_client(&self, config: OAuthConfig, callback_url: &str) -> OAuthClient {
        match self {
            OAuthProvider::Google => OAuthClient::new(
                config,
                "https://accounts.google.com/o/oauth2/v2/auth",
                "https://www.googleapis.com/oauth2/v3/token",
                callback_url,
                &[&"https://www.googleapis.com/auth/userinfo.email"],
                EmailRequest {
                    request_endpoint: RequestEndpoint::FormatUrl(|token| {
                        format!(
                            "https://www.googleapis.com/oauth2/v1/userinfo?access_token={}",
                            token
                        )
                    }),
                    email_retrieve: |obj| root_json_str_parse(obj, "email"),
                },
            ),
            OAuthProvider::Github => OAuthClient::new(
                config,
                "https://github.com/login/oauth/authorize",
                "https://github.com/login/oauth/access_token",
                callback_url,
                &[&"user"],
                EmailRequest {
                    request_endpoint: RequestEndpoint::Bearer(
                        "https://api.github.com/user/emails".into(),
                    ),
                    email_retrieve: |res| {
                        #[derive(Deserialize)]
                        struct EmailResponse {
                            primary: bool,
                            email: String,
                        }

                        if let Ok(emails) = serde_json::from_value::<Vec<EmailResponse>>(res) {
                            for email in emails {
                                if email.primary {
                                    return Some(email.email);
                                }
                            }
                        }

                        None
                    },
                },
            ),
            OAuthProvider::Discord => OAuthClient::new(
                config,
                "https://discord.com/oauth2/authorize",
                "https://discord.com/api/oauth2/token",
                callback_url,
                &[&"identify", "email"],
                EmailRequest {
                    request_endpoint: RequestEndpoint::Bearer(
                        "https://discord.com/api/v10/users/@me".into(),
                    ),
                    email_retrieve: |obj| root_json_str_parse(obj, "email"),
                },
            ),
        }
    }
}

struct EmailRequest {
    request_endpoint: RequestEndpoint,
    /// Email retriever using result data.
    email_retrieve: fn(serde_json::Value) -> Option<String>,
}

/// User request endpoint configuration for getting email.
enum RequestEndpoint {
    /// Format URL with token as argument.
    FormatUrl(fn(&str) -> String),
    /// Automatically use token in `Authorization` header.
    Bearer(String),
}

pub struct OAuthClient {
    http_client: reqwest::Client,
    client: BasicClient,
    scopes: Vec<Scope>,
    email_request: EmailRequest,
}

impl OAuthClient {
    fn new(
        oauth_config: OAuthConfig,
        auth_url: &str,
        token_url: &str,
        redirect_url: &str,
        scopes: &[&str],
        email_request: EmailRequest,
    ) -> Self {
        let auth_url = AuthUrl::new(auth_url.to_string()).unwrap();
        let token_url = TokenUrl::new(token_url.to_string()).unwrap();

        Self {
            http_client: reqwest::Client::builder()
                .user_agent("Backpack")
                .build()
                .unwrap(),
            client: BasicClient::new(
                ClientId::new(oauth_config.client_id),
                Some(ClientSecret::new(oauth_config.client_secret)),
                auth_url,
                Some(token_url),
            )
            .set_redirect_uri(RedirectUrl::new(redirect_url.into()).expect("Invalid redirect URL")),
            scopes: scopes
                .to_vec()
                .iter()
                .map(|f| Scope::new(f.to_string()))
                .collect(),
            email_request,
        }
    }

    /// Initiate an oauth login with provided scopes.
    /// Start the login session by redirecting the user to the provider URL.
    pub fn login(&self) -> ServiceResult<oauth2::url::Url> {
        // TODO: PKCE verification.

        // Generate the authorization URL to which we'll redirect the user.
        let (authorize_url, _csrf_state) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scopes(self.scopes.clone())
            .url();

        Ok(authorize_url)
    }

    /// Use auth params provided by the provider to get the email.
    pub async fn get_email(&self, oauth_request: &AuthRequest) -> ServiceResult<String> {
        let code = AuthorizationCode::new(oauth_request.code.clone());

        // Exchange the code with a token.
        let token = match self
            .client
            .exchange_code(code)
            .request_async(async_http_client)
            .await
        {
            Ok(v) => v,
            Err(e) => return Err(ServiceError::ServerError(e.into())),
        };

        let response = match &self.email_request.request_endpoint {
            RequestEndpoint::FormatUrl(formatter) => self
                .http_client
                .get(formatter(token.access_token().secret())),
            RequestEndpoint::Bearer(url) => self
                .http_client
                .get(url)
                .bearer_auth(token.access_token().secret()),
        }
        .send()
        .await
        .map_err(|e| ServiceError::ServerError(e.into()))?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| ServiceError::ServerError(e.into()))?;

        match (self.email_request.email_retrieve)(response) {
            Some(v) => Ok(v),
            None => Err(ServiceError::ServerError(anyhow::anyhow!(
                "OAuth provider was misconfigured."
            ))),
        }
    }
}

/// Extract any string field from the root of a [`serde_json::Value`].
/// This returns [`None`] if this fails.
fn root_json_str_parse(object: serde_json::Value, field: &str) -> Option<String> {
    let object = serde_json::from_value::<serde_json::Value>(object);

    if let Ok(serde_json::Value::Object(object)) = object {
        if let Some(serde_json::Value::String(str)) = object.get(field) {
            return Some(str.to_owned());
        }
    }

    None
}
