use std::{borrow::BorrowMut, sync::Mutex};

use crate::state::{State};
use crate::models::user::*;

use actix_web::*;

pub fn get_routes() -> Scope {
    web::scope("/user/")
        .service(create)
}

#[post("create")]
pub async fn create(state: web::Data<State>, query: web::Query<UserCreateForm>) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}