use crate::{
    database::entity::{files, settings},
    models::AppInfo,
    services::{user::UserService, ServiceError},
};
use actix_web::{get, web, HttpResponse, Responder, Scope};
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait};

pub mod admin;
pub mod application;
pub mod auth;
pub mod file;
pub mod user;

pub fn get_routes() -> Scope {
    web::scope("").service(info)
}

/// Get public server configuration
#[utoipa::path(
    context_path = "/api",
    tag = "server",
    responses(
        (status = 200, body = AppInfo)
    )
)]
#[get("/info")]
async fn info(
    user_service: web::Data<UserService>,
    database: web::Data<DatabaseConnection>,
) -> impl Responder {
    match settings::Entity::find_by_id(true)
        .one(database.as_ref())
        .await
        .map_err(|e| ServiceError::DbErr(e))
    {
        Ok(settings) => {
            let settings = settings.unwrap();
            HttpResponse::Ok().json(AppInfo::new(
                settings::Model {
                    one_row_enforce: true,
                    app_name: settings.app_name,
                    app_description: settings.app_description,
                    color: settings.color,
                },
                user_service.invite_only(),
                user_service.smtp_enabled(),
                match files::Entity::find()
                    .count(database.as_ref())
                    .await
                    .map_err(|e| ServiceError::DbErr(e))
                {
                    Ok(v) => v,
                    Err(e) => return e.to_response(),
                },
            ))
        }
        Err(e) => e.to_response(),
    }
}
