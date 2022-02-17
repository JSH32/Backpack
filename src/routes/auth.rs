use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::Utc;
use time::OffsetDateTime;

use crate::{
    database,
    models::{auth::BasicAuthForm, Error, MessageResponse, Response},
    state::State,
    util::{
        self,
        auth::{auth_role, create_jwt_string, Auth},
    },
};

use actix_web::{cookie::Cookie, http::StatusCode, post, web, HttpResponse, Responder, Scope};

pub fn get_routes() -> Scope {
    web::scope("/auth").service(basic).service(logout)
}

/// Login with email and password
#[post("/basic")]
async fn basic(
    state: web::Data<State>,
    form: web::Json<BasicAuthForm>,
) -> Response<impl Responder> {
    let user_data = match if util::EMAIL_REGEX.is_match(&form.auth) {
        state.database.get_user_by_email(&form.auth).await
        // TODO: block usernames from having @ or weird characters
    } else {
        state.database.get_user_by_username(&form.auth).await
    } {
        Ok(v) => v,
        Err(err) => {
            // Check if the error was due to not finding or due to something else (worth throwing a 500 for)
            if let database::error::Error::SqlxError(err) = err {
                if let sqlx::Error::RowNotFound = err {
                    return Ok(MessageResponse::new(
                        StatusCode::BAD_REQUEST,
                        "Invalid credentials provided!",
                    )
                    .http_response());
                } else {
                    return Err(Error::from(err));
                }
            } else {
                return Err(Error::from(err));
            }
        }
    };

    // Check if password is valid to password hash
    if Argon2::default()
        .verify_password(
            form.password.as_bytes(),
            &PasswordHash::new(&user_data.password)?,
        )
        .is_ok()
    {
        return Ok(
            MessageResponse::new(StatusCode::BAD_REQUEST, "Invalid credentials provided!")
                .http_response(),
        );
    }

    let expire_time = (Utc::now() + chrono::Duration::weeks(1)).timestamp();
    let jwt = create_jwt_string(
        &user_data.id,
        None,
        &state
            .base_url
            .host()
            .expect("BASE_URL must have host included"),
        Some(expire_time),
        &state.jwt_key,
    )?;

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
#[post("/logout")]
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
