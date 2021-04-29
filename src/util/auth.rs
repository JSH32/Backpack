use actix_web::{Error, HttpMessage, HttpRequest, web::Data};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Serialize, Deserialize};

use crate::state::State;
use crate::models::MessageResponse;
use crate::models::user::UserData;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JWTClaims {
    pub iss: String, // Issuer
    pub iat: i64, // Issued at

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<i64>, // Expire

    pub user_id: i32, // ID user refers to

    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<i32> // Token ID, if the token was a forever token
}

/// Generate auth middleware for a UserRole.
/// This implementation will allow the specified role or lower access level roles to access a resource
macro_rules! define_auth {
    ($name:ident, $role_enum:expr) => {
        // Authentication middleware for this role. This will also work for roles at a lower access level
        pub struct $name(pub $crate::models::user::UserData);

        impl actix_web::FromRequest for $name {
            type Error = actix_web::Error;
            type Future = std::pin::Pin<Box<dyn futures::Future<Output = Result<$name, actix_web::Error>>>>;
            type Config = ();

            fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
                let req = req.clone();

                Box::pin(async move {
                    let user_data = match $crate::util::auth::get_auth_data(req).await {
                        Ok(user_data) => user_data,
                        Err(err) => return Err(err)
                    };

                    if user_data.role < $role_enum {
                        return Err(actix_web::Error::from($crate::models::MessageResponse::unauthorized_error()))
                    }

                    Ok($name(user_data))
                })
            }
        }
    }
}

/// Get data from user based on request
async fn get_auth_data(req: HttpRequest) -> Result<UserData, actix_web::Error> {
    let state = req.app_data::<Data<State>>().expect("State was not found");

    let jwt_token = match req.cookie("auth-token") {
        Some(jwt_token) => jwt_token,
        // Token could not be found
        None => return Err(Error::from(MessageResponse::unauthorized_error()))
    };

    // Try to verify token
    let claims = match decode::<JWTClaims>(jwt_token.value(), &DecodingKey::from_secret(state.jwt_key.as_ref()), &Validation::default()) {
        Ok(claims) => claims.claims,
        Err(_) => return Err(Error::from(MessageResponse::unauthorized_error()))
    };

    let user = match state.database.get_user_by_id(claims.user_id).await {
        Ok(data) => data,
        Err(_) => return Err(Error::from(MessageResponse::unauthorized_error()))
    };

    // Check if it is perm JWT token
    if let Some(token_id) = claims.token_id {
        match state.database.get_token_by_id(token_id).await {
            Ok(token_data) => {
                // Check if perm JWT token belongs to user
                if token_data.user_id != user.id {
                    return Err(Error::from(MessageResponse::unauthorized_error()))
                }
            }
            Err(_) => return Err(Error::from(MessageResponse::unauthorized_error()))
        };
    }

    Ok(user)
}

// Auth middleware defines
pub mod middleware {
    use crate::models::user::UserRole;

    define_auth!(User, UserRole::User);
    define_auth!(Admin, UserRole::Admin);
}

// Sign a JWT token and get a string
pub fn create_jwt_string(user_id: i32, token_id: Option<i32>, issuer: &str, expiration: Option<i64>, key: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = JWTClaims {
        iss: issuer.into(),
        exp: expiration,
        iat: Utc::now().timestamp(),
        user_id: user_id,
        token_id: token_id
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(key.as_ref()))
}
