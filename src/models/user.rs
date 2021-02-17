use serde::{Serialize, Deserialize};
use actix_web::{Error, FromRequest, HttpMessage, HttpRequest, dev::Payload, http::StatusCode, web::Data};
use futures::future::{Ready, err, ok};
use jwt::{VerifyWithKey, RegisteredClaims};

use crate::state::State;
use crate::models::Response;

#[derive(Deserialize)]
pub struct UserCreateForm {
    pub username: String,
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize, sqlx::Type)]
#[sqlx(rename = "role")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all(serialize  = "lowercase"))]
#[serde(rename_all(deserialize  = "UPPERCASE"))]
pub enum UserRole {
    USER,
    ADMIN
}

#[derive(Serialize)]
pub struct UserData {
    #[serde(skip_serializing)]
    pub id: i32,

    #[serde(skip_serializing)]
    pub password: String,

    pub username: String,
    pub email: String,
    pub verified: bool,
    pub role: UserRole
}


/// Authentication middleware implementation
impl FromRequest for UserData {
    type Error = Error;
    type Future = Ready<Result<UserData, Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let state = req.app_data::<Data<State>>().expect("State was not found for authentication middleware");
        let jwt_token = match req.cookie("auth-token") {
            Some(jwt_token) => jwt_token,
            // Token could not be found
            None => return err(Error::from(Response::new_message(StatusCode::UNAUTHORIZED, true, "You are not authorized to make this request!")))
        };

        // Try to verify token
        let claim: RegisteredClaims = match jwt_token.value().verify_with_key(&state.jwt_key) {
            Ok(claim) => claim,
            // Token verification failed
            Err(_) => return err(Error::from(Response::new_message(StatusCode::UNAUTHORIZED, true, "You are not authorized to make this request!")))
        };

        let result = futures::executor::block_on(state.database.get_user_by_id(claim.subject.unwrap().parse().unwrap())).unwrap();

        ok(result)
    }
}