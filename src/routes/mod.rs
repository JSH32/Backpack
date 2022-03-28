use crate::{
    database::entity::settings,
    models::{AppInfo, Response},
    state::State,
};
use actix_web::{get, web, HttpResponse, Responder, Scope};
use sea_orm::{EntityTrait, ActiveEnum};

pub mod admin;
pub mod application;
pub mod auth;
pub mod file;
pub mod user;

pub fn get_routes() -> Scope {
    web::scope("/").service(info)
}

#[get("info")]
async fn info(state: web::Data<State>) -> Response<impl Responder> {
    let settings = settings::Entity::find_by_id(true)
        .one(&state.database)
        .await?
        .unwrap();

    Ok(HttpResponse::Ok().json(AppInfo {
        app_name: settings.app_name,
        app_description: settings.app_description,
        color: settings.color.to_value(),
        invite_only: state.invite_only,
        smtp: state.smtp_client.is_some(),
    }))
}
