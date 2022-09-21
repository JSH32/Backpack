use std::pin::Pin;

use derive_more::Display;
use futures::Future;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::config::OAuthConfig;
use crate::database::entity::sea_orm_active_enums::AuthMethod;
use crate::models::AuthRequest;
use crate::services::{ServiceError, ServiceResult};

/// All OAuth providers.
#[derive(Debug, Display, Clone, Copy)]
pub enum OAuthProvider {
    Google,
    Github,
    Discord,
}

impl Into<AuthMethod> for OAuthProvider {
    fn into(self) -> AuthMethod {
        match self {
            Self::Google => AuthMethod::Google,
            Self::Github => AuthMethod::Github,
            Self::Discord => AuthMethod::Discord,
        }
    }
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
                &[
                    &"https://www.googleapis.com/auth/userinfo.email",
                    "https://www.googleapis.com/auth/userinfo.profile",
                ],
                |ctx| {
                    Box::pin(async move {
                        #[derive(Deserialize)]
                        struct Response {
                            id: String,
                            email: String,
                        }

                        match ctx
                            .make_request::<Response>(
                                &format!(
                                    "https://www.googleapis.com/oauth2/v1/userinfo?access_token={}",
                                    ctx.token
                                ),
                                None,
                            )
                            .await
                        {
                            Ok(v) => Some(OAuthUserData {
                                id: v.id,
                                username: v.email[..v.email.find('@')?].to_owned(),
                                email: v.email,
                            }),
                            Err(_) => None,
                        }
                    })
                },
            ),
            OAuthProvider::Github => OAuthClient::new(
                config,
                "https://github.com/login/oauth/authorize",
                "https://github.com/login/oauth/access_token",
                callback_url,
                &[&"user"],
                |ctx| {
                    Box::pin(async move {
                        #[derive(Deserialize)]
                        struct UserResponse {
                            id: usize,
                            login: String,
                        }

                        let user = match ctx
                            .make_request::<UserResponse>(
                                "https://api.github.com/user",
                                Some(&ctx.token),
                            )
                            .await
                        {
                            Ok(v) => v,
                            Err(_) => return None,
                        };

                        #[derive(Deserialize)]
                        struct EmailResponse {
                            primary: bool,
                            email: String,
                        }

                        // Find the users email.
                        match ctx
                            .make_request::<Vec<EmailResponse>>(
                                "https://api.github.com/user/emails",
                                Some(&ctx.token),
                            )
                            .await
                        {
                            Ok(emails) => match emails.iter().find(|e| e.primary) {
                                Some(v) => Some(OAuthUserData {
                                    id: user.id.to_string(),
                                    username: user.login,
                                    email: v.email.clone(),
                                }),
                                None => None,
                            },
                            Err(_) => None,
                        }
                    })
                },
            ),
            OAuthProvider::Discord => OAuthClient::new(
                config,
                "https://discord.com/oauth2/authorize",
                "https://discord.com/api/oauth2/token",
                callback_url,
                &[&"identify", "email"],
                |ctx| {
                    Box::pin(async move {
                        match ctx
                            .make_request::<OAuthUserData>(
                                "https://discord.com/api/v10/users/@me",
                                Some(&ctx.token),
                            )
                            .await
                        {
                            Ok(v) => Some(v),
                            Err(_) => None,
                        }
                    })
                },
            ),
        }
    }
}

/// OAuth request context.
struct RequestContext {
    client: reqwest::Client,
    pub token: String,
}

impl RequestContext {
    /// Make a simple request and return as a [`ServiceResult`].
    pub async fn make_request<T: DeserializeOwned>(
        &self,
        url: &str,
        bearer: Option<&str>,
    ) -> ServiceResult<T> {
        let mut req_builder = self.client.get(url);

        if let Some(bearer) = bearer {
            req_builder = req_builder.bearer_auth(bearer)
        }

        Ok(req_builder
            .send()
            .await
            .map_err(|e| ServiceError::ServerError(e.into()))?
            .json::<T>()
            .await
            .map_err(|e| ServiceError::ServerError(e.into()))?)
    }
}

/// Should return (unique_identifier, email).
/// If [`None`] then failed.
type DataRequest = fn(RequestContext) -> Pin<Box<dyn Future<Output = Option<OAuthUserData>>>>;

#[derive(Deserialize)]
pub struct OAuthUserData {
    pub id: String,
    pub email: String,
    pub username: String,
}

pub struct OAuthClient {
    http_client: reqwest::Client,
    client: BasicClient,
    scopes: Vec<Scope>,
    data_request: DataRequest,
}

impl OAuthClient {
    fn new(
        oauth_config: OAuthConfig,
        auth_url: &str,
        token_url: &str,
        redirect_url: &str,
        scopes: &[&str],
        data_request: DataRequest,
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
            data_request,
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

    /// Use auth params provided by the provider to get the user data.
    pub async fn get_user_data(&self, oauth_request: &AuthRequest) -> ServiceResult<OAuthUserData> {
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

        match (self.data_request)(RequestContext {
            client: self.http_client.clone(),
            token: token.access_token().secret().to_string(),
        })
        .await
        {
            Some(v) => Ok(v),
            None => Err(ServiceError::ServerError(anyhow::anyhow!(
                "OAuth provider was misconfigured."
            ))),
        }
    }
}
