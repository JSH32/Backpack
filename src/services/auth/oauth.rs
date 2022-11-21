use std::pin::Pin;

use derive_more::Display;
use futures::Future;
use moka::future::Cache;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::config::OAuthConfig;
use crate::database::entity::sea_orm_active_enums::AuthMethod;
use crate::models::OAuthRequest;
use crate::services::{ServiceError, ServiceResult};

/// All OAuth providers.
#[derive(Debug, Display, Clone, Copy, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
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

#[derive(Clone, Debug)]
pub struct OAuthState {
    /// User ID to attach account to.
    pub user_id: Option<String>,
    /// Optional redirect, will redirect to this URL after request.
    pub redirect: Option<String>,
    /// Should the redirect be included in params with the redirect URL.
    pub include_redirect: bool,
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
                                // Google accounts are always verified since they are the source of the email.
                                verified: true,
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
                            verified: bool,
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
                                    verified: v.verified,
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
    pub verified: bool,
}

#[derive(Debug)]
pub struct OAuthClient {
    http_client: reqwest::Client,
    client: BasicClient,
    scopes: Vec<Scope>,
    data_request: DataRequest,
    /// Cache stores CSRF token secrets to OAuth state.
    /// Values should be removed on usage and automatically after a timeout period.
    state_cache: Cache<String, OAuthState>,
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
            // 10 minute expiry time.
            state_cache: Cache::builder()
                .time_to_live(std::time::Duration::from_secs_f32((60 * 10) as f32))
                .build(),
        }
    }

    /// Initiate an oauth login with provided scopes.
    /// Start the login session by redirecting the user to the provider URL.
    pub async fn login(
        &self,
        user_id: Option<String>,
        redirect: Option<String>,
        include_redirect: bool,
    ) -> ServiceResult<oauth2::url::Url> {
        // TODO: PKCE verification.

        // Generate the authorization URL to which we'll redirect the user.
        let (authorize_url, csrf_state) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scopes(self.scopes.clone())
            .url();

        self.state_cache
            .insert(
                csrf_state.secret().to_string(),
                OAuthState {
                    user_id,
                    redirect,
                    include_redirect,
                },
            )
            .await;

        Ok(authorize_url)
    }

    /// Use auth params provided by the provider to get the user data.
    pub async fn get_user_data(
        &self,
        oauth_request: &OAuthRequest,
    ) -> ServiceResult<(OAuthUserData, OAuthState)> {
        let code = AuthorizationCode::new(oauth_request.code.clone());
        let state = CsrfToken::new(oauth_request.state.clone());

        let oauth_state = match self.state_cache.get(state.secret()) {
            Some(oauth_state) => {
                self.state_cache.invalidate(state.secret()).await;
                oauth_state
            }
            None => return Err(ServiceError::Unauthorized("Invalid Csrf token.".into())),
        };

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
            Some(v) => Ok((v, oauth_state)),
            None => Err(ServiceError::ServerError(anyhow::anyhow!(
                "OAuth provider was misconfigured."
            ))),
        }
    }
}
