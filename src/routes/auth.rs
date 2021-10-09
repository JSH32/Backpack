use chrono::Utc;
use time::OffsetDateTime;

use crate::{
    models::{auth::BasicAuthForm, MessageResponse},
    state::State,
    util::{
        self,
        auth::{auth_role, create_jwt_string, Auth},
    },
};

use actix_web::{cookie::Cookie, http::StatusCode, post, web, HttpResponse, Responder, Scope};

pub fn get_routes() -> Scope {
    web::scope("/auth/").service(basic).service(logout)
}

/// Login with email and password
#[post("basic")]
async fn basic(
    state: web::Data<State>,
    form: web::Json<BasicAuthForm>,
) -> Result<HttpResponse, MessageResponse> {
    let user_data = if util::EMAIL_REGEX.is_match(&form.auth) {
        state.database.get_user_by_email(&form.auth).await
    } else {
        state.database.get_user_by_username(&form.auth).await
    }
    .map_err(|_| MessageResponse::new(StatusCode::BAD_REQUEST, "Invalid credentials provided!"))?;

    // Check if password is valid to password hash
    match argon2::verify_encoded(&user_data.password, form.password.as_bytes()) {
        Ok(matches) => {
            if !matches {
                return Err(MessageResponse::new(
                    StatusCode::BAD_REQUEST,
                    "Invalid credentials provided!",
                ));
            }
        }
        Err(_) => return Err(MessageResponse::internal_server_error()),
    };

    let expire_time = (Utc::now() + chrono::Duration::weeks(1)).timestamp();
    let jwt = match create_jwt_string(
        &user_data.id,
        None,
        &state.base_url,
        Some(expire_time),
        &state.jwt_key,
    ) {
        Ok(jwt) => jwt,
        Err(_) => return Err(MessageResponse::internal_server_error()),
    };

    // Set JWT token as cookie
    Ok(HttpResponse::Ok()
        .cookie(
            Cookie::build("auth-token", jwt)
                .secure(false)
                .http_only(true)
                .path("/")
                .expires(OffsetDateTime::from_unix_timestamp(expire_time))
                .finish(),
        )
        .json(user_data))
}

/// Remove httponly cookie
#[post("logout")]
async fn logout(_: Auth<auth_role::User, true, false>) -> impl Responder {
    HttpResponse::Ok()
        .cookie(
            Cookie::build("auth-token", "")
                .secure(false)
                .http_only(true)
                .path("/")
                // Cookie expires instantly when issued, will remove the cookie
                .expires(OffsetDateTime::from_unix_timestamp(Utc::now().timestamp()))
                .finish(),
        )
        .json(MessageResponse::new(
            StatusCode::OK,
            "Successfully logged out",
        ))
}
