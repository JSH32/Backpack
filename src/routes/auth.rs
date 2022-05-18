use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::Utc;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    database::entity::users,
    models::{auth::BasicAuthForm, MessageResponse, Response, UserData},
    state::State,
    util::{
        self,
        auth::{auth_role, create_jwt_string, verify_user, Auth},
    },
};

use actix_web::{
    cookie::{time::OffsetDateTime, Cookie},
    http::StatusCode,
    post, web, HttpResponse, Responder, Scope,
};

pub fn get_routes() -> Scope {
    web::scope("/auth").service(basic).service(logout)
}

/// Login with email and password
#[post("/basic")]
async fn basic(
    state: web::Data<State>,
    form: web::Json<BasicAuthForm>,
) -> Response<impl Responder> {
    let mut user_data = match if util::EMAIL_REGEX.is_match(&form.auth) {
        users::Entity::find()
            .filter(users::Column::Email.eq(form.auth.to_owned()))
            .one(&state.database)
            .await
    } else {
        users::Entity::find()
            .filter(users::Column::Username.eq(form.auth.to_owned()))
            .one(&state.database)
            .await
    }? {
        Some(v) => v,
        None => {
            return MessageResponse::ok(StatusCode::BAD_REQUEST, "Invalid credentials provided!");
        }
    };

    // Check if password is valid to password hash
    if !Argon2::default()
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

    // Verify user if SMTP is disabled
    if let (None, false) = (&state.smtp_client, user_data.verified) {
        verify_user(&mut user_data, &state.database).await?;
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
                .expires(OffsetDateTime::from_unix_timestamp(expire_time).unwrap())
                .finish(),
        )
        .json(UserData::from(user_data)))
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
                .expires(OffsetDateTime::from_unix_timestamp(Utc::now().timestamp()).unwrap())
                .finish(),
        )
        .json(MessageResponse::new(
            StatusCode::OK,
            "Successfully logged out",
        ))
}
