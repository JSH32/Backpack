use actix_web::{HttpResponse, Responder, Scope, get, web};

use crate::{models::IDQuery, state::State};

pub fn get_routes() -> Scope {
    web::scope("/file/")
        .service(info)
}

#[get("/info")]
async fn info(state: web::Data<State>, id: web::Query<IDQuery>) -> impl Responder {
    HttpResponse::Ok()
}