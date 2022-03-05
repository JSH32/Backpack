use crate::{
    database::entity::settings,
    models::{AppInfo, Response},
    state::State,
};
use actix_web::{get, web, HttpResponse, Responder, Scope};
use sea_orm::EntityTrait;

pub mod application;
pub mod auth;
pub mod file;
pub mod user;
pub mod registration_key;

pub fn get_routes() -> Scope {
    web::scope("/").service(info)
}

#[get("info")]
async fn info(state: web::Data<State>) -> Response<impl Responder> {
    Ok(HttpResponse::Ok().json(AppInfo::from(
        settings::Entity::find_by_id(true)
            .one(&state.database)
            .await?
            .unwrap(),
    )))
}
