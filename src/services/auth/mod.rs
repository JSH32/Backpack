use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand::{rngs::OsRng, Rng};
use sea_orm::{ColumnTrait, Condition};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use url::Url;

use crate::{
    config::OAuthConfig,
    database::entity::{applications, auth_methods, sea_orm_active_enums::AuthMethod, users},
    models::{OAuthRequest, TokenResponse},
};

use self::{
    auth_method::AuthMethodService,
    oauth::{OAuthClient, OAuthProvider},
};

use super::{
    application::ApplicationService, prelude::DataService, user::UserService, ServiceError,
    ServiceResult, ToOption,
};

pub mod auth_method;
pub mod oauth;

/// Handles authentication and validation.
pub struct AuthService {
    auth_method_service: Arc<AuthMethodService>,
    user_service: Arc<UserService>,
    // TODO: Figure out how to cleanly avoid this circular dependency.
    application_service: Arc<RwLock<Option<Arc<ApplicationService>>>>,
    api_url: actix_http::Uri,
    jwt_key: String,
    /// Root URL of client.
    pub client_url: String,

    google_oauth_client: Option<OAuthClient>,
    github_oauth_client: Option<OAuthClient>,
    discord_oauth_client: Option<OAuthClient>,
}

impl AuthService {
    pub fn new(
        auth_method_service: Arc<AuthMethodService>,
        user_service: Arc<UserService>,
        application_service: Arc<RwLock<Option<Arc<ApplicationService>>>>,
        api_url: &str,
        jwt_key: &str,
        client_url: &str,
        google_oauth: Option<OAuthConfig>,
        github_oauth: Option<OAuthConfig>,
        discord_oauth: Option<OAuthConfig>,
    ) -> Self {
        Self {
            auth_method_service,
            user_service,
            application_service,
            api_url: api_url.parse::<actix_http::Uri>().unwrap(),
            jwt_key: jwt_key.into(),
            client_url: client_url.into(),
            google_oauth_client: match google_oauth {
                Some(config) => Some(
                    OAuthProvider::Google
                        .new_client(config, &format!("{}/api/auth/google/callback", api_url)),
                ),
                None => None,
            },
            github_oauth_client: match github_oauth {
                Some(config) => Some(
                    OAuthProvider::Github
                        .new_client(config, &format!("{}/api/auth/github/callback", api_url)),
                ),
                None => None,
            },
            discord_oauth_client: match discord_oauth {
                Some(config) => Some(
                    OAuthProvider::Discord
                        .new_client(config, &format!("{}/api/auth/discord/callback", api_url)),
                ),
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
    pub async fn password_auth(&self, auth: &str, password: &str) -> ServiceResult<TokenResponse> {
        let user = self.user_service.get_by_identifier(auth).await?;
        let method = self
            .auth_method_service
            .get_auth_method(&user.id, AuthMethod::Password)
            .await?;

        validate_password(&method.value, password)?;

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

        match self.user_service.by_id(claims.sub).await.to_option()? {
            Some(mut user) => {
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
                    let application_service =
                        self.application_service.read().unwrap().clone().unwrap();

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
            None => Err(ServiceError::unauthorized()),
        }
    }

    /// Create a new JWT for the user
    pub fn new_jwt(
        &self,
        user_id: &str,
        application_id: Option<String>,
    ) -> ServiceResult<TokenResponse> {
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

        Ok(TokenResponse { token: jwt })
    }

    /// Check if the OAuth provider is enabled.
    pub fn oauth_enabled(&self, provider_type: OAuthProvider) -> bool {
        self.get_oauth_client(provider_type).is_ok()
    }

    /// Initiate an oauth login.
    /// Start the login session by redirecting the user to the provider URL.
    pub async fn oauth_login(
        &self,
        provider_type: OAuthProvider,
        user_id: Option<String>,
        redirect: Option<String>,
        include_redirect: bool,
    ) -> ServiceResult<oauth2::url::Url> {
        self.get_oauth_client(provider_type)?
            .login(user_id, redirect, include_redirect)
            .await
    }

    /// Use auth params provided by the provider to get a JWT token.
    /// If a user dowes not exist with these parameters, create the user.
    /// Returns new JWT key.
    pub async fn oauth_authenticate(
        &self,
        provider_type: OAuthProvider,
        auth_request: &OAuthRequest,
    ) -> ServiceResult<(TokenResponse, Option<String>)> {
        let (oauth_data, oauth_state) = self
            .get_oauth_client(provider_type)?
            .get_user_data(auth_request)
            .await?;

        // let full_redirect = Uri::from(oauth_state.redirect)

        // Get a user based on auth method.
        let user = match self
            .auth_method_service
            .get_user_by_value(
                provider_type.clone().into(),
                &oauth_data.id,
                Some(oauth_data.username.clone()),
            )
            .await?
        {
            // User found.
            Some(user) => {
                // User was already linked.
                if let Some(_) = oauth_state.user_id {
                    return Err(ServiceError::InvalidData(
                        "That account was already linked.".into(),
                    ));
                }

                user
            }
            // Check if email already exists.
            None => {
                // We don't care about verified email if we are linking to specific account.
                if let Some(user_id) = oauth_state.user_id {
                    if let Some(user) = self.user_service.by_id(user_id).await.to_option()? {
                        self.auth_method_service
                            .create_auth_method(
                                &user.id,
                                provider_type.into(),
                                Some(oauth_data.username),
                                &oauth_data.id,
                            )
                            .await?;

                        let token = self.new_jwt(&user.id, None)?;
                        return Ok((
                            token.clone(),
                            make_redirect_url(
                                oauth_state.redirect,
                                &token.token,
                                oauth_state.include_redirect,
                            ),
                        ));
                    }
                }

                // No linking or creating an account with an unverified email.
                if !oauth_data.verified {
                    return Err(ServiceError::InvalidData(
                        "Email was not verified on the account used.".into(),
                    ));
                }

                match self
                    .user_service
                    .by_condition(
                        Condition::any().add(users::Column::Email.eq(oauth_data.email.clone())),
                    )
                    .await
                {
                    Ok(user) => {
                        let auth_method: AuthMethod = provider_type.into();
                        // Make sure an existing account with the same email and auth method doesn't exist.
                        if let Some(_) = self
                            .auth_method_service
                            .by_condition(
                                Condition::all()
                                    .add(auth_methods::Column::UserId.eq(user.id.clone()))
                                    .add(auth_methods::Column::AuthMethod.eq(auth_method.clone())),
                            )
                            .await
                            .to_option()?
                        {
                            return Err(ServiceError::InvalidData(
                                "Existing account already has that email and method.".into(),
                            ));
                        };

                        self.auth_method_service
                            .create_auth_method(
                                &user.id,
                                auth_method,
                                Some(oauth_data.username),
                                &oauth_data.id,
                            )
                            .await?;

                        user
                    }
                    Err(e) => match e {
                        // Make a new user.
                        ServiceError::NotFound(_) => {
                            self.user_service
                                .create_user(
                                    self.new_unique_username(&oauth_data.username).await?,
                                    oauth_data.email,
                                    (
                                        provider_type.into(),
                                        oauth_data.id,
                                        Some(oauth_data.username),
                                    ),
                                    None,
                                )
                                .await?
                        }
                        _ => return Err(e),
                    },
                }
            }
        };

        let token = self.new_jwt(&user.id, None)?;
        Ok((
            token.clone(),
            make_redirect_url(
                oauth_state.redirect,
                &token.token,
                oauth_state.include_redirect,
            ),
        ))
    }

    /// Validate or attempt to create a valid username based on an existing username.
    /// This can, by a really small chance, create a non-unique username.
    async fn new_unique_username(&self, username: &str) -> ServiceResult<String> {
        let mut username = username.to_owned();

        // Try adding a 4 random digits.
        if username.len() < 5 {
            username.push_str(&random_digit_str(4).to_string());
        }

        if let Some(_) = self
            .user_service
            .get_by_identifier(&username)
            .await
            .to_option()?
        {
            // Found, try adding 4 digits.
            username.push_str(&random_digit_str(4).to_string());
        };

        Ok(username)
    }

    fn get_oauth_client(&self, provider_type: OAuthProvider) -> ServiceResult<&OAuthClient> {
        let provider = match provider_type {
            OAuthProvider::Google => &self.google_oauth_client,
            OAuthProvider::Github => &self.github_oauth_client,
            OAuthProvider::Discord => &self.discord_oauth_client,
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

fn random_digit_str(digits: u32) -> i32 {
    let p = 10i32.pow(digits - 1);
    rand::thread_rng().gen_range(p..10 * p)
}

fn make_redirect_url(
    redirect: Option<String>,
    token: &str,
    include_redirect: bool,
) -> Option<String> {
    match redirect {
        Some(v) => {
            match if include_redirect {
                let mut map = HashMap::new();
                map.insert("token", token.to_owned());

                Url::parse_with_params(&v, map)
            } else {
                Url::parse(&v)
            } {
                Ok(v) => Some(v.to_string()),
                Err(_) => None,
            }
        }
        None => None,
    }
}
