use actix_web::{delete, get, http::StatusCode, post, web, Responder, Scope};
use sea_orm::{prelude::*, Condition};

use crate::{
    database::entity::applications,
    internal::auth::{auth_role, Auth},
    models::application::*,
    services::{
        application::ApplicationService, prelude::DataService, ToMessageResponse, ToPageResponse,
        ToResponse,
    },
};

pub fn get_routes() -> Scope {
    web::scope("/application")
        .service(list)
        .service(info)
        .service(create)
        .service(delete)
        .service(token)
}

/// Get token by application ID
/// - Minimum required role: `user`
/// - Allow unverified users: `false`
/// - Application token allowed: `false`
#[utoipa::path(
    context_path = "/api/application",
    tag = "application",
    responses(
        (status = 200, body = TokenResponse),
        (status = 404, body = MessageResponse, description = "Application not found")
    ),
    params(
        ("application_id" = str, Path, description = "Application ID to get token for"),
    ),
    security(("apiKey" = [])),
)]
#[get("/{application_id}/token")]
async fn token(
    service: web::Data<ApplicationService>,
    application_id: web::Path<String>,
    user: Auth<auth_role::User>,
) -> impl Responder {
    service
        .generate_token(&application_id, Some(&user.id))
        .await
        .to_response::<TokenResponse>(StatusCode::OK)
}

/// Get all applications
/// - Minimum required role: `user`
/// - Allow unverified users: `false`
/// - Application token allowed: `false`
#[utoipa::path(
    context_path = "/api/application",
    tag = "application",
    responses((status = 200, body = ApplicationPage)),
    params(
        ("page_number" = u64, Path, description = "Page to get applications by (starts at 1)"),
    ),
    security(("apiKey" = [])),
)]
#[get("/list/{page_number}")]
async fn list(
    service: web::Data<ApplicationService>,
    page_number: web::Path<usize>,
    user: Auth<auth_role::User>,
) -> impl Responder {
    service
        .get_page(
            *page_number,
            5,
            Some(Condition::any().add(applications::Column::UserId.eq(user.id.to_owned()))),
        )
        .await
        .to_page_response::<ApplicationData>(StatusCode::OK)
}

/// Get token info
/// - Minimum required role: `user`
/// - Allow unverified users: `false`
/// - Application token allowed: `false`
#[utoipa::path(
    context_path = "/api/application",
    tag = "application",
    responses(
        (status = 200, body = ApplicationData),
        (status = 401, body = MessageResponse)
    ),
    security(("apiKey" = [])),
)]
#[get("/{application_id}")]
async fn info(
    service: web::Data<ApplicationService>,
    user: Auth<auth_role::User>,
    application_id: web::Path<String>,
) -> impl Responder {
    service
        .by_condition(
            Condition::all()
                .add(applications::Column::UserId.eq(user.id.to_owned()))
                .add(applications::Column::Id.eq(application_id.to_owned())),
        )
        .await
        .to_response::<ApplicationData>(StatusCode::OK)
}

/// Create an application
/// - Minimum required role: `user`
/// - Allow unverified users: `false`
/// - Application token allowed: `false`
#[utoipa::path(
    context_path = "/api/application",
    tag = "application",
    responses(
        (status = 200, body = ApplicationData),
        (status = 400, body = MessageResponse, description = "Token limit reached or invalid name"),
    ),
    request_body = ApplicationCreate,
    security(("apiKey" = [])),
)]
#[post("")]
async fn create(
    service: web::Data<ApplicationService>,
    user: Auth<auth_role::User>,
    form: web::Json<ApplicationCreate>,
) -> impl Responder {
    service
        .create_application(&user.id, &form.name)
        .await
        .to_response::<ApplicationData>(StatusCode::OK)
}

/// Delete an application
/// - Minimum required role: `user`
/// - Allow unverified users: `false`
/// - Application token allowed: `false`
#[utoipa::path(
    context_path = "/api/application",
    tag = "application",
    responses(
        (status = 200, body = MessageResponse, description = "Application was deleted"),
        (status = 401, body = MessageResponse, description = "Unauthorized or token does not exist"),
    ),
    security(("apiKey" = [])),
)]
#[delete("/{application_id}")]
async fn delete(
    service: web::Data<ApplicationService>,
    user: Auth<auth_role::User>,
    application_id: web::Path<String>,
) -> impl Responder {
    service
        .delete(
            application_id.to_string(),
            false,
            Some(Condition::all().add(applications::Column::UserId.eq(user.id.to_owned()))),
        )
        .await
        .to_message_response(StatusCode::OK)
}
