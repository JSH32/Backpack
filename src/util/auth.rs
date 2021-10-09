use actix_web::{http::StatusCode, web::Data, Error, FromRequest, HttpRequest};

use chrono::Utc;

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use serde::{Deserialize, Serialize};

use crate::{
    models::{user::UserData, MessageResponse, UserRole},
    state::State,
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JWTClaims {
    iss: String, // Issuer
    iat: i64,    // Issued at

    #[serde(skip_serializing_if = "Option::is_none")]
    exp: Option<i64>, // Expire

    user_id: String, // User ID the token refers to

    #[serde(skip_serializing_if = "Option::is_none")]
    application_id: Option<String>, // Application ID, if the token was an application token
}

/// TODO: Replace the use of this entirely with const_generics when possible
/// Not currently possible due to a rust compiler bug in the nightly build
///
/// https://github.com/rust-lang/rust/issues/84737
pub trait Role {
    const LEVEL: UserRole;
}

macro_rules! define_role {
    ($name:ident, $variant:expr) => {
        pub struct $name;
        impl $crate::util::auth::Role for $name {
            const LEVEL: $crate::models::user::UserRole = $variant;
        }
    };
}

// Define all auth roles
pub mod auth_role {
    use crate::models::user::UserRole;

    define_role!(User, UserRole::User);
    define_role!(Admin, UserRole::Admin);
}

pub struct Auth<R: Role, const ALLOW_UNVERIFIED: bool, const ALLOW_APPLICATION: bool> {
    pub user: UserData,
    _r: std::marker::PhantomData<R>,
}

impl<R: Role, const ALLOW_UNVERIFIED: bool, const ALLOW_APPLICATION: bool> FromRequest
    for Auth<R, ALLOW_UNVERIFIED, ALLOW_APPLICATION>
{
    type Error = Error;
    type Future = std::pin::Pin<
        Box<
            dyn futures::Future<
                Output = Result<Auth<R, ALLOW_UNVERIFIED, ALLOW_APPLICATION>, Error>,
            >,
        >,
    >;
    type Config = ();

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let req = req.clone();

        Box::pin(async move {
            let (user_data, is_application) = match get_auth_data(req, ALLOW_UNVERIFIED).await {
                Ok(user_data) => user_data,
                Err(err) => return Err(err),
            };

            if is_application && !ALLOW_APPLICATION {
                return Err(Error::from(MessageResponse::unauthorized_error()));
            }

            if user_data.role < R::LEVEL {
                return Err(Error::from(MessageResponse::unauthorized_error()));
            }

            Ok(Auth {
                user: user_data,
                _r: std::marker::PhantomData,
            })
        })
    }
}

fn get_token(req: &HttpRequest) -> Option<String> {
    match req.cookie("auth-token") {
        Some(cookie) => Some(cookie.value().to_string()),
        // Token could not be found
        None => match req.headers().get("Authorization") {
            Some(header) => match header.to_str() {
                Ok(value) => {
                    // Auth type must be bearer
                    if value.starts_with("Bearer ") {
                        Some(value.trim_start_matches("Bearer ").to_string())
                    } else {
                        None
                    }
                }
                Err(_) => None,
            },
            None => None,
        },
    }
}

/// Get data from user based on request
async fn get_auth_data(
    req: HttpRequest,
    allow_unverified: bool,
) -> Result<(UserData, bool), actix_web::Error> {
    let state = req.app_data::<Data<State>>().expect("State was not found");

    let jwt_token = get_token(&req).ok_or(Error::from(MessageResponse::unauthorized_error()))?;

    // Try to verify token
    let claims = decode::<JWTClaims>(
        &jwt_token,
        &DecodingKey::from_secret(state.jwt_key.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| Error::from(MessageResponse::unauthorized_error()))?
    .claims;

    let mut user = state
        .database
        .get_user_by_id(&claims.user_id)
        .await
        .map_err(|_| Error::from(MessageResponse::unauthorized_error()))?;

    // Block user out if unverified is false
    if state.smtp_client.is_some() && !user.verified && !allow_unverified {
        return Err(Error::from(MessageResponse::new(
            StatusCode::UNAUTHORIZED,
            "You need to verify your email",
        )));
    }

    if !user.verified {
        match state.smtp_client {
            Some(_) => {
                if !allow_unverified {
                    return Err(Error::from(MessageResponse::new(
                        StatusCode::UNAUTHORIZED,
                        "You need to verify your email",
                    )));
                }
            }
            None => {
                // Delete all verifications for the user
                if let Err(_) = state.database.delete_verification(&user.id).await {
                    return Err(Error::from(MessageResponse::internal_server_error()));
                }

                // Verify the user
                if let Err(_) = state.database.verify_user(&user.id, true).await {
                    return Err(Error::from(MessageResponse::internal_server_error()));
                }

                user.verified = true;
            }
        }
    }

    let mut is_application = false;

    // Check if it is perm JWT token
    if let Some(application_id) = claims.application_id {
        match state.database.get_application_by_id(&application_id).await {
            Ok(application_data) => {
                is_application = true;

                // Check if perm JWT token belongs to user
                if application_data.user_id != user.id {
                    return Err(Error::from(MessageResponse::unauthorized_error()));
                }
            }
            // Application has been deleted so it's ID does not exist anymore, invalid token
            Err(_) => return Err(Error::from(MessageResponse::unauthorized_error())),
        };
    }

    Ok((user, is_application))
}

// Sign a JWT token and get a string
pub fn create_jwt_string(
    user_id: &str,
    application_id: Option<String>,
    issuer: &str,
    expiration: Option<i64>,
    key: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = JWTClaims {
        iss: issuer.into(),
        exp: expiration,
        iat: Utc::now().timestamp(),
        user_id: user_id.to_string(),
        application_id: application_id,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(key.as_ref()),
    )
}
