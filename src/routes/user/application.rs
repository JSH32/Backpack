use actix_web::{get, http::StatusCode, web, Responder, Scope};
use sea_orm::{prelude::*, Condition};

use crate::{
    database::entity::applications,
    internal::auth::{auth_role, Auth},
    models::application::*,
    services::{application::ApplicationService, prelude::DataService, ToPageResponse},
};

pub fn get_routes() -> Scope {
    web::scope("/applications").service(list)
}

/// Get all applications owned by a user.
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
#[get("/{page_number}")]
async fn list(
    service: web::Data<ApplicationService>,
    page_number: web::Path<usize>,
    user: Auth<auth_role::User>,
    user_id: web::Path<String>,
) -> impl Responder {
    // TODO: Authorized `get_page`
    service
        .get_page(
            *page_number,
            5,
            Some(Condition::any().add(applications::Column::UserId.eq(user.id.to_owned()))),
        )
        .await
        .to_page_response::<ApplicationData>(StatusCode::OK)
}
