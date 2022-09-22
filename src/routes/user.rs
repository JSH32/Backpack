use actix_web::{
    delete, get, http::StatusCode, patch, post, put, web, HttpResponse, Responder, Scope,
};

use crate::{
    database::entity::sea_orm_active_enums::AuthMethod,
    internal::auth::{
        auth_role, AllowApplication, AllowUnregistered, AllowUnverified, Auth, DenyApplication,
    },
    models::{
        MessageResponse, RegistrationParams, UpdateUserSettings, UserCreateForm, UserData,
        UserDeleteForm,
    },
    services::{user::UserService, ToResponse},
};

pub fn get_routes() -> Scope {
    web::scope("/user")
        .service(create)
        .service(delete)
        .service(settings)
        .service(info)
        .service(resend_verify)
        .service(verify)
        .service(register_key)
}

/// Get current user information
/// - Minimum required role: `user`
/// - Allow unverified users: `true`
/// - Application token allowed: `true`
#[utoipa::path(
    context_path = "/api/user",
    tag = "user",
    responses(
        (status = 200, body = UserData)
    ),
    security(("apiKey" = []))
)]
#[get("")]
async fn info(
    user: Auth<auth_role::User, AllowUnverified, AllowApplication, AllowUnregistered>,
) -> impl Responder {
    HttpResponse::Ok().json(UserData::from(user.user))
}

/// Change user settings
/// - Minimum required role: `user`
/// - Allow unverified users: `true`
/// - Application token allowed: `false`
#[utoipa::path(
    context_path = "/api/user",
    tag = "user",
    responses(
        (status = 200, body = UserData),
        (status = 400, body = MessageResponse),
        (status = 409, body = MessageResponse)
    ),
    security(("apiKey" = [])),
    request_body = UpdateUserSettings
)]
#[put("/settings")]
async fn settings(
    service: web::Data<UserService>,
    form: web::Json<UpdateUserSettings>,
    user: Auth<auth_role::User, AllowUnverified, DenyApplication, AllowUnregistered>,
) -> impl Responder {
    service
        .update_settings(
            &user,
            form.0.email,
            form.0.username,
            form.0.new_password,
            form.0.current_password,
        )
        .await
        .to_response::<UserData>(StatusCode::OK)
}

/// Register account using a registration key.
/// This is only required on services with `invite_only` enabled.
#[utoipa::path(
    context_path = "/api/user",
    tag = "user",
    responses(
        (status = 200, body = UserData),
        (status = 400, body = MessageResponse),
    ),
    params(RegistrationParams)
)]
#[get("/register")]
async fn register_key(
    service: web::Data<UserService>,
    user: Auth<auth_role::User, AllowUnverified, DenyApplication, AllowUnregistered>,
    params: web::Query<RegistrationParams>,
) -> impl Responder {
    service
        .register_user(&user, &params.key)
        .await
        .to_response::<UserData>(StatusCode::OK)
}

/// Create a new user
#[utoipa::path(
    context_path = "/api/user",
    tag = "user",
    responses(
        (status = 200, body = UserData),
        (status = 400, body = MessageResponse),
        (status = 409, body = MessageResponse)
    ),
    request_body = UserCreateForm
)]
#[post("")]
async fn create(
    service: web::Data<UserService>,
    form: web::Json<UserCreateForm>,
) -> impl Responder {
    match service
        .create_user(
            form.0.username,
            form.0.email,
            (AuthMethod::Password, form.0.password, None),
            form.0.registration_key,
        )
        .await
    {
        Ok(_) => MessageResponse::new(StatusCode::OK, "User has successfully been created")
            .http_response(),
        Err(e) => e.to_response(),
    }
}

/// Resend a verification code to the email
/// - Minimum required role: `user`
/// - Allow unverified users: `true`
/// - Application token allowed: `false`
///
/// This will be disabled if `smtp` is disabled in server settings
#[utoipa::path(
    context_path = "/api/user",
    tag = "user",
    responses(
        (status = 200, body = MessageResponse),
        (status = 409, body = MessageResponse, description = "Already verified"),
        (status = 410, body = MessageResponse, description = "SMTP is disabled")
    ),
    security(("apiKey" = [])),
)]
#[patch("/verify/resend")]
async fn resend_verify(
    service: web::Data<UserService>,
    user: Auth<auth_role::User, AllowUnverified, DenyApplication, AllowUnregistered>,
) -> impl Responder {
    match service.resend_verification(&user).await {
        Ok(v) => MessageResponse::new(
            StatusCode::OK,
            &format!("Verification email resent to {}", v),
        )
        .http_response(),
        Err(e) => e.to_response(),
    }
}

/// Verify using a verification code
///
/// This will be disabled if `smtp` is disabled in server settings
#[utoipa::path(
    context_path = "/api/user",
    tag = "user",
    responses(
        (status = 200, body = MessageResponse),
        (status = 400, body = MessageResponse, description = "Invalid verification code"),
        (status = 410, body = MessageResponse, description = "SMTP is disabled")
    ),
    params(
        ("code" = str, Path, description = "Verification code to verify"),
    )
)]
#[patch("/verify/{code}")]
async fn verify(service: web::Data<UserService>, code: web::Path<String>) -> impl Responder {
    match service.verify_by_code(&code).await {
        Ok(_) => MessageResponse::new(StatusCode::OK, "User has been verified").http_response(),
        Err(e) => e.to_response(),
    }
}

/// Delete a user and all files owned by the user
/// - Minimum required role: `user`
/// - Allow unverified users: `true`
/// - Application token allowed: `false`
#[utoipa::path(
    context_path = "/api/user",
    tag = "user",
    responses(
        (status = 200, body = MessageResponse, description = "User was deleted"),
        (status = 400, body = MessageResponse, description = "Incorrect password")
    ),
    request_body(content = UserDeleteForm, description = "Verify your password"),
    security(("apiKey" = [])),
)]
#[delete("")]
async fn delete(
    service: web::Data<UserService>,
    user: Auth<auth_role::User, AllowUnverified, DenyApplication, AllowUnregistered>,
    form: web::Json<UserDeleteForm>,
) -> impl Responder {
    match service.delete(&user, form.password.clone()).await {
        Ok(_) => MessageResponse::new(StatusCode::OK, "User has been deleted").http_response(),
        Err(e) => e.to_response(),
    }
}
