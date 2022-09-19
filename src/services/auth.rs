use actix_http::Uri;
use anyhow::anyhow;
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::Utc;
use derive_more::Display;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use rand::rngs::OsRng;
use sea_orm::{ColumnTrait, Condition};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{
    config::OAuthConfig,
    database::entity::{applications, users},
    models::AuthRequest,
};

use super::{
    application::ApplicationService, prelude::DataService, user::UserService, ServiceError,
    ServiceResult,
};

/// All OAuth providers.
#[derive(Debug, Display)]
pub enum OAuthProvider {
    Google,
    Github,
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

struct OAuthClient {
    http_client: reqwest::Client,
    client: BasicClient,
    scopes: Vec<Scope>,
    email_request: EmailRequest,
}

impl OAuthClient {
    pub fn new(
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

    /// Initiate an oauth login.
    /// Start the login session by redirecting the user to the provider URL.
    fn login(&self) -> ServiceResult<oauth2::url::Url> {
        // TODO: PKCE verification.

        // Generate the authorization URL to which we'll redirect the user.
        let (authorize_url, _csrf_state) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scopes(self.scopes.clone())
            .url();

        Ok(authorize_url)
    }

    /// Use auth params provided by the provider to get a JWT token.
    /// Returns user email.
    async fn auth(&self, oauth_request: &AuthRequest) -> ServiceResult<String> {
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
            None => Err(ServiceError::ServerError(anyhow!(
                "OAuth provider was misconfigured."
            ))),
        }
    }
}

/// Handles authentication and validation.
pub struct AuthService {
    user_service: Arc<UserService>,
    // TODO: Figure out how to avoid this circular dependency.
    application_service: Arc<RwLock<Option<Arc<ApplicationService>>>>,
    api_url: Uri,
    jwt_key: String,
    /// Root URL of client.
    pub client_url: String,

    google_oauth_client: Option<OAuthClient>,
    github_oauth_client: Option<OAuthClient>,
}

impl AuthService {
    pub fn new(
        user_service: Arc<UserService>,
        application_service: Arc<RwLock<Option<Arc<ApplicationService>>>>,
        api_url: &str,
        jwt_key: &str,
        client_url: &str,
        google_oauth: Option<OAuthConfig>,
        github_oauth: Option<OAuthConfig>,
    ) -> Self {
        Self {
            user_service,
            application_service,
            api_url: api_url.parse::<Uri>().unwrap(),
            jwt_key: jwt_key.into(),
            client_url: client_url.into(),
            google_oauth_client: match google_oauth {
                Some(config) => Some(OAuthClient::new(
                    config,
                    "https://accounts.google.com/o/oauth2/v2/auth",
                    "https://www.googleapis.com/oauth2/v3/token",
                    &format!("{}/api/auth/google/auth", api_url),
                    &[&"https://www.googleapis.com/auth/userinfo.email"],
                    EmailRequest {
                        request_endpoint: RequestEndpoint::FormatUrl(|token| {
                            format!(
                                "https://www.googleapis.com/oauth2/v1/userinfo?access_token={}",
                                token
                            )
                        }),
                        email_retrieve: |res| {
                            if let serde_json::Value::Object(obj) = res {
                                if let Some(serde_json::Value::String(email)) = obj.get("email") {
                                    Some(email.to_owned())
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        },
                    },
                )),
                None => None,
            },
            github_oauth_client: match github_oauth {
                Some(config) => Some(OAuthClient::new(
                    config,
                    "https://github.com/login/oauth/authorize",
                    "https://github.com/login/oauth/access_token",
                    &format!("{}/api/auth/github/auth", api_url),
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
                )),
                None => None,
            },
        }
    }

    /// Authenticate a user using their name/email and password.
    ///
    /// * `username` - User identifier (email or username)
    /// * `password` - User password
    ///
    /// Returns JWT token response.
    pub async fn password_auth(
        &self,
        auth: &str,
        password: &str,
    ) -> ServiceResult<crate::models::TokenResponse> {
        let user = self.user_service.get_by_identifier(auth).await?;

        validate_password(&user.password, password)?;

        if self.user_service.smtp_enabled() {
            self.user_service.verify_user(&user).await?;
        }

        self.new_jwt(&user.id, None)
    }

    /// Validate JWT with specific parameters.
    ///
    /// # Arguments
    ///
    /// * `allow_unverified` - Allow unverified users.
    pub async fn validate_jwt(
        &self,
        allow_unverified: bool,
        jwt_token: &str,
    ) -> ServiceResult<(users::Model, Option<applications::Model>)> {
        let mut validation = Validation::default();

        // Application tokens might not have expiration date so it's not required.
        validation.required_spec_claims.remove("exp");

        // Try to verify token
        let claims = decode::<JwtClaims>(
            &jwt_token,
            &DecodingKey::from_secret(self.jwt_key.as_ref()),
            &validation,
        )
        .map_err(|_| ServiceError::Unauthorized("You are not authorized".into()))?
        .claims;

        let mut user = self.user_service.by_id(claims.sub).await?;

        if !user.verified {
            if self.user_service.smtp_enabled() {
                if !allow_unverified {
                    return Err(ServiceError::Unauthorized(
                        "You need to verify your email".into(),
                    ));
                }
            } else {
                self.user_service.verify_user(&user).await?;
                user.verified = true;
            }
        }

        let mut application = None;

        if let Some(application_id) = claims.application_id {
            let application_service = self.application_service.read().unwrap().clone().unwrap();
            match application_service.by_id(application_id).await {
                Ok(v) => {
                    // Check if perm JWT token belongs to user
                    if v.user_id != user.id {
                        return Err(ServiceError::unauthorized());
                    }

                    // Update last accessed
                    application_service.update_accessed(&v.id).await?;
                    application = Some(v);
                }
                Err(e) => match e {
                    ServiceError::NotFound(_) => return Err(ServiceError::unauthorized()),
                    // Any other error should actually be an error.
                    e => return Err(e),
                },
            }
        }

        Ok((user, application))
    }

    /// Create a new JWT for the user
    pub fn new_jwt(
        &self,
        user_id: &str,
        application_id: Option<String>,
    ) -> ServiceResult<crate::models::TokenResponse> {
        let expire_time = (Utc::now() + chrono::Duration::weeks(1)).timestamp();

        let claims = JwtClaims {
            iss: self
                .api_url
                .host()
                .expect("API_URL must have host included")
                .into(),
            exp: Some(expire_time),
            iat: Utc::now().timestamp(),
            sub: user_id.to_string(),
            application_id: application_id,
        };

        let jwt = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_key.as_ref()),
        )
        .map_err(|e| ServiceError::ServerError(e.into()))?;

        Ok(crate::models::TokenResponse { token: jwt })
    }

    /// Initiate an oauth login.
    /// Start the login session by redirecting the user to the provider URL.
    pub fn oauth_login(&self, provider_type: OAuthProvider) -> ServiceResult<oauth2::url::Url> {
        self.get_client(provider_type)?.login()
    }

    /// Use auth params provided by the provider to get a JWT token.
    /// Returns new JWT key.
    pub async fn oauth_authenticate(
        &self,
        provider_type: OAuthProvider,
        auth_request: &AuthRequest,
    ) -> ServiceResult<crate::models::TokenResponse> {
        let email = self.get_client(provider_type)?.auth(auth_request).await?;

        let user = self
            .user_service
            .by_condition(Condition::all().add(users::Column::Email.eq(email)))
            .await?;

        self.new_jwt(&user.id, None)
    }

    fn get_client(&self, provider_type: OAuthProvider) -> ServiceResult<&OAuthClient> {
        let provider = match provider_type {
            OAuthProvider::Google => &self.google_oauth_client,
            OAuthProvider::Github => &self.github_oauth_client,
        };

        match provider {
            Some(v) => Ok(v),
            None => Err(ServiceError::InvalidData(format!(
                "{} OAuth provider was not enabled for this service.",
                provider_type.to_string()
            ))),
        }
    }
}

// Validate a password.
pub fn validate_password(hash: &str, password: &str) -> ServiceResult<()> {
    // Check if the users password is correct
    if !Argon2::default()
        .verify_password(
            password.as_bytes(),
            &PasswordHash::new(&hash).map_err(|e| ServiceError::ServerError(e.into()))?,
        )
        .is_ok()
    {
        return Err(ServiceError::InvalidData(
            "Incorrect credentials provided".into(),
        ));
    }

    Ok(())
}

/// Data stored in JWT token.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JwtClaims {
    /// Issuer (the domain).
    iss: String,

    /// Issued at.
    iat: i64,

    /// Expiration date.
    /// This should be [`None`] if `application_id` is [`Some`].
    #[serde(skip_serializing_if = "Option::is_none")]
    exp: Option<i64>,

    /// Subject, user ID the token refers to.
    sub: String,

    /// ID of the Application.
    /// This should be [`Some`] if the token was an application token.
    #[serde(skip_serializing_if = "Option::is_none")]
    application_id: Option<String>,
}

/// Validate a password and hash it.
pub fn new_password(password: &str) -> ServiceResult<String> {
    let password_length = password.len();
    if password_length < 6 {
        Err(ServiceError::InvalidData(
            "Password too short (minimum 6 characters)".into(),
        ))
    } else if password_length > 128 {
        Err(ServiceError::InvalidData(
            "Password too long (maximum 128 characters)".into(),
        ))
    } else {
        Ok(Argon2::default()
            .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
            .map_err(|e| ServiceError::ServerError(e.into()))?
            .to_string())
    }
}
