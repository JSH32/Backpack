use serde::{Deserialize};
use actix_web::{Error, FromRequest, HttpMessage, HttpRequest, dev::Payload, error::ErrorUnauthorized, web::Data};
use futures::future::{Ready, err, ok};
use jwt::{VerifyWithKey, RegisteredClaims};

use crate::state::State;

use super::*;

#[derive(Deserialize)]
pub struct BasicAuthForm {
    pub email: String,
    pub password: String
}

pub struct UserID(pub i32);

/// Authentication middleware implementation
impl FromRequest for UserID {
    type Error = Error;
    type Future = Ready<Result<UserID, Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let state = req.app_data::<Data<State>>().expect("State was not found for authentication middleware");
        let jwt_token = match req.cookie("auth-token") {
            Some(jwt_token) => jwt_token,
            // Token could not be found
            None => return err(Error::from(Response::new_message(StatusCode::UNAUTHORIZED, true, "You are not authorized to make this request!")))
        };

        let claim: RegisteredClaims = match jwt_token.value().verify_with_key(&state.jwt_key) {
            Ok(claim) => claim,
            Err(_) => return err(Error::from(Response::new_message(StatusCode::UNAUTHORIZED, true, "You are not authorized to make this request!")))
        };

        ok(UserID(claim.subject.unwrap().parse().unwrap()))
    }
}