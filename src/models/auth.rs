use std::pin::Pin;

use serde::{Deserialize, Serialize};
use actix_web::{Error, FromRequest, HttpMessage, HttpRequest, dev::Payload, web::Data};
use futures::{Future};
use jwt::{VerifyWithKey, RegisteredClaims};

use crate::state::State;
use super::{MessageResponse};
use super::user::{UserData, UserRole};

#[derive(Deserialize)]
pub struct BasicAuthForm {
    pub email: String,
    pub password: String
}

/// Generate auth middleware for a UserRole.
/// This implementation will allow the specified role or lower access level roles to access a resource
macro_rules! define_auth {
    ($name:ident, $role_enum:expr) => {
        #[doc = "Authentication middleware for this role. This will work for roles at a lower access level"]
        pub struct $name(pub UserData);

        impl FromRequest for $name {
            type Error = Error;
            type Future = Pin<Box<dyn Future<Output = Result<$name, Error>>>>;
            type Config = ();

            fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
                let req = req.clone();

                Box::pin(async move {
                    let user_data = match get_auth_data(req).await {
                        Ok(user_data) => user_data,
                        Err(err) => return Err(err)
                    };

                    if user_data.role < $role_enum {
                        return Err(Error::from(MessageResponse::unauthorized_error()))
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
    let claim: RegisteredClaims = match jwt_token.value().verify_with_key(&state.jwt_key) {
        Ok(claim) => claim,
        // Token verification failed
        Err(_) => return Err(Error::from(MessageResponse::unauthorized_error()))
    };

    let user_id = match claim.subject {
        Some(data) => {
            match data.parse() {
                Ok(parsed) => parsed,
                Err(_) => return Err(Error::from(MessageResponse::internal_server_error()))
            }
        },
        None => return Err(Error::from(MessageResponse::internal_server_error()))
    };

    match state.database.get_user_by_id(user_id).await {
        Ok(data) => Ok(data),
        Err(_) => return Err(Error::from(MessageResponse::internal_server_error()))
    }
}

// Auth middleware defines
define_auth!(User, UserRole::User);
define_auth!(Admin, UserRole::Admin);