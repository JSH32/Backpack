//! Authentication logic using [`FromRequest`].
//!
//! TODO: Replace the use of traits used in generics entirely with const_generics when possible.
//! Not currently possible due to a rust compiler bug in the nightly build.
//! https://github.com/rust-lang/rust/issues/84737

use actix_web::{http::StatusCode, web::Data, Error, FromRequest, HttpRequest};

use chrono::Utc;

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::{
    database::entity::{applications, users, verifications},
    models::{MessageResponse, UserRole},
    state::State,
};

/// Data stored in JWT token.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Claims {
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

pub trait Role {
    const LEVEL: UserRole;
}

macro_rules! define_role {
    ($name:ident, $variant:expr) => {
        pub struct $name;
        impl $crate::internal::auth::Role for $name {
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

/// Define an auth option which can be used in generic parameters.
macro_rules! define_option {
    ($option:ident, $allow_name:ident, $deny_name:ident) => {
        pub trait $option {
            const ALLOW: bool;
        }

        pub struct $allow_name;
        impl $option for $allow_name {
            const ALLOW: bool = true;
        }

        pub struct $deny_name;
        impl $option for $deny_name {
            const ALLOW: bool = false;
        }
    };
}

define_option!(VerifiedOpt, AllowUnverified, DenyUnverified);
define_option!(ApplicationOpt, AllowApplication, DenyApplication);

pub struct Auth<R: Role, VOpt: VerifiedOpt = DenyUnverified, AOpt: ApplicationOpt = DenyApplication>
{
    pub user: users::Model,
    _r: std::marker::PhantomData<R>,
    _v: std::marker::PhantomData<VOpt>,
    _a: std::marker::PhantomData<AOpt>,
}

impl<R: Role, VOpt: VerifiedOpt, AOpt: ApplicationOpt> Deref for Auth<R, VOpt, AOpt> {
    type Target = users::Model;

    fn deref(&self) -> &users::Model {
        &self.user
    }
}

impl<R: Role, VOpt: VerifiedOpt, AOpt: ApplicationOpt> FromRequest for Auth<R, VOpt, AOpt> {
    type Error = Error;
    type Future =
        std::pin::Pin<Box<dyn futures::Future<Output = Result<Auth<R, VOpt, AOpt>, Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let req = req.clone();

        Box::pin(async move {
            let (user_data, is_application) = get_auth_data(req, VOpt::ALLOW as bool).await?;

            if (is_application && !AOpt::ALLOW)
                || (UserRole::from(user_data.role.clone()) < R::LEVEL)
            {
                return Err(Error::from(MessageResponse::unauthorized_error()));
            }

            Ok(Auth {
                user: user_data,
                _r: std::marker::PhantomData,
                _v: std::marker::PhantomData,
                _a: std::marker::PhantomData,
            })
        })
    }
}

fn get_token(req: &HttpRequest) -> Option<String> {
    match req.headers().get("Authorization") {
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
    }
}

/// Get data from user based on request.
/// If the authentication request failed this will return an [`actix_web::Error`].
async fn get_auth_data(
    req: HttpRequest,
    allow_unverified: bool,
) -> Result<(users::Model, bool), actix_web::Error> {
    let state = req.app_data::<Data<State>>().expect("State was not found");

    let jwt_token = get_token(&req).ok_or(Error::from(MessageResponse::unauthorized_error()))?;

    let mut validation = Validation::default();
    validation.required_spec_claims.remove("exp"); // Application tokens might not have expiration date.

    // Try to verify token
    let claims = decode::<Claims>(
        &jwt_token,
        &DecodingKey::from_secret(state.jwt_key.as_ref()),
        &validation,
    )
    .map_err(|_| Error::from(MessageResponse::unauthorized_error()))?
    .claims;

    let mut user = users::Entity::find_by_id(claims.sub)
        .one(&state.database)
        .await
        .map_err(|_| Error::from(MessageResponse::unauthorized_error()))?
        .ok_or(Error::from(MessageResponse::unauthorized_error()))?;

    // Error if user isn't verified and verification is required
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
            None => verify_user(&mut user, &state.database)
                .await
                .map_err(|e| Error::from(MessageResponse::internal_server_error(&e.to_string())))?,
        }
    }

    let mut application = None;

    // Check if it is perm JWT token
    if let Some(application_id) = claims.application_id {
        match applications::Entity::find_by_id(application_id)
            .one(&state.database)
            .await
            .map_err(|err| Error::from(MessageResponse::internal_server_error(&err.to_string())))?
        {
            Some(application_data) => {
                application = Some(application_data.id);

                // Check if perm JWT token belongs to user
                if application_data.user_id != user.id {
                    return Err(Error::from(MessageResponse::unauthorized_error()));
                }
            }
            // Application has been deleted so it's ID does not exist anymore, invalid token
            None => return Err(Error::from(MessageResponse::unauthorized_error())),
        }
    }

    // Update last accessed
    if let Some(application_id) = &application {
        applications::ActiveModel {
            id: Set(application_id.to_owned()),
            last_accessed: Set(Utc::now()),
            ..Default::default()
        }
        .update(&state.database)
        .await
        .map_err(|err| Error::from(MessageResponse::internal_server_error(&err.to_string())))?;
    }

    Ok((user, application.is_some()))
}

pub async fn verify_user(
    user: &mut users::Model,
    db: &DatabaseConnection,
) -> Result<(), anyhow::Error> {
    verifications::Entity::delete_many()
        .filter(verifications::Column::UserId.eq(user.id.to_owned()))
        .exec(db)
        .await?;

    users::ActiveModel {
        id: Set(user.id.to_owned()),
        verified: Set(true),
        ..Default::default()
    }
    .update(db)
    .await?;

    user.verified = true;

    Ok(())
}

// Sign a JWT token and get a string
pub fn create_jwt_string(
    user_id: &str,
    application_id: Option<String>,
    issuer: &str,
    expiration: Option<i64>,
    key: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims {
        iss: issuer.into(),
        exp: expiration,
        iat: Utc::now().timestamp(),
        sub: user_id.to_string(),
        application_id: application_id,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(key.as_ref()),
    )
}
