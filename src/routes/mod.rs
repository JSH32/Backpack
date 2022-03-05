use crate::{
    database::entity::settings,
    models::{AppInfo, Response},
    state::State,
};
use actix_web::{get, web, HttpResponse, Responder, Scope};
use sea_orm::EntityTrait;
use std::env;

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
    let mut data = AppInfo::from(
        settings::Entity::find_by_id(true)
            .one(&state.database)
            .await?
            .unwrap(),
    );
    data.set_commit(env::var("COMMIT").unwrap_or(String::from("0000000000000000000000000000000")));
    Ok(HttpResponse::Ok().json(data))
}
