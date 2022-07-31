use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::Utc;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    database::entity::users,
    internal::{
        self,
        auth::{create_jwt_string, verify_user},
        response::Response,
    },
    models::{auth::BasicAuthForm, MessageResponse, TokenResponse},
    state::State,
};

use actix_web::{http::StatusCode, post, web, HttpResponse, Responder, Scope};

pub fn get_routes() -> Scope {
    web::scope("/auth").service(basic)
}

/// Login with email and password
#[utoipa::path(
    context_path = "/api/auth",
    tag = "authentication",
    responses(
        (status = 200, body = TokenResponse),
        (status = 400, body = MessageResponse, description = "Invalid credentials"),
    ),
    request_body(content = BasicAuthForm)
)]
#[post("/basic")]
async fn basic(
    state: web::Data<State>,
    form: web::Json<BasicAuthForm>,
) -> Response<impl Responder> {
    let mut user_data = match if internal::EMAIL_REGEX.is_match(&form.auth) {
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
            return MessageResponse::ok(StatusCode::BAD_REQUEST, "Invalid credentials provided!")
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
            .api_url
            .host()
            .expect("API_URL must have host included"),
        Some(expire_time),
        &state.jwt_key,
    )?;

    Ok(HttpResponse::Ok().json(TokenResponse { token: jwt }))
}
