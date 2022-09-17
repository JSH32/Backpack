use actix_http::Uri;
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

use crate::{
    database::entity::{applications, users},
    models::TokenResponse,
};

use super::{
    application::ApplicationService, prelude::DataService, user::UserService, ServiceError,
    ServiceResult,
};

/// Handles authentication and validation.
pub struct AuthService {
    user_service: Arc<UserService>,
    // TODO: Figure out how to avoid this circular dependency.
    application_service: Arc<RwLock<Option<Arc<ApplicationService>>>>,
    api_url: Uri,
    jwt_key: String,
}

impl AuthService {
    pub fn new(
        user_service: Arc<UserService>,
        application_service: Arc<RwLock<Option<Arc<ApplicationService>>>>,
        api_url: Uri,
        jwt_key: String,
    ) -> Self {
        Self {
            user_service,
            application_service,
            api_url,
            jwt_key,
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
