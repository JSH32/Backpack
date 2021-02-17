use std::pin::Pin;

use serde::Deserialize;
use actix_web::{Error, FromRequest, HttpMessage, HttpRequest, dev::Payload, web::Data};
use futures::{Future, future::{Ready, err, ok}};
use jwt::{VerifyWithKey, RegisteredClaims};

use crate::state::State;
use super::MessageResponse;
use super::user::UserData;

#[derive(Deserialize)]
pub struct BasicAuthForm {
    pub email: String,
    pub password: String
}

/// Authentication data middleware
pub struct Auth {
    pub user_data: UserData,
    pub is_token: bool
}

/// Auth but only works on JWT authenticated user, will not work on tokens
pub struct AuthJWT(pub UserData);

/// JWT only authentication middleware implementation
impl FromRequest for AuthJWT {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<AuthJWT, Error>>>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let req = req.clone();
        let mut payload = payload.take();
        Box::pin(async move { 
            Auth::from_request(&req, &mut payload).await.map(|x| {
                if x.is_token {
                    Error::from(MessageResponse::unauthorized_error());
                }
                AuthJWT(x.user_data)
            })
        })
    }
}

/// Authentication middleware implementation
impl FromRequest for Auth {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Auth, Error>>>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();

        Box::pin(async move {
            let state = req.app_data::<Data<State>>().expect("State was not found for authentication middleware");

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

            let result = match state.database.get_user_by_id(user_id).await {
                Ok(data) => data,
                Err(_) => return Err(Error::from(MessageResponse::internal_server_error()))
            };

            Ok(Auth{
                user_data: result,
                is_token: false,
            })
        })
    }
}