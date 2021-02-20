use serde::{Deserialize};
use actix_web::{Error, HttpMessage, HttpRequest, web::Data};
use jwt::{VerifyWithKey, RegisteredClaims};

use crate::state::State;
use super::MessageResponse;
use super::user::UserData;

#[derive(Deserialize)]
pub struct BasicAuthForm {
    pub email: String,
    pub password: String
}