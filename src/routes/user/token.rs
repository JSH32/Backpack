use crate::{models::*, state::State};
use crate::util::auth;

use actix_web::*;
use actix_web::http::StatusCode;

pub fn get_routes() -> Scope {
    web::scope("/token/")
}

#[get("list")]
async fn list(state: web::Data<State>, auth: auth::middleware::User) -> impl Responder {
    HttpResponse::new(StatusCode::NOT_IMPLEMENTED)
}

#[post("info")]
async fn info(state: web::Data<State>, auth: auth::middleware::User, data: web::Json<IDQuery>) -> impl Responder {
    HttpResponse::new(StatusCode::NOT_IMPLEMENTED)
}

#[post("create")]
async fn create(state: web::Data<State>, auth: auth::middleware::User, data: web::Json<TokenData>) -> impl Responder {
    HttpResponse::new(StatusCode::NOT_IMPLEMENTED)
}

#[get("delete")]
async fn delete(state: web::Data<State>, auth: auth::middleware::User, data: web::Json<IDQuery>) -> impl Responder {
    HttpResponse::new(StatusCode::NOT_IMPLEMENTED)
}