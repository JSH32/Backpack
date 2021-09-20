use actix_web::{HttpResponse, Responder, Scope, get, web};

use crate::{models::IDQuery, state::State};

pub fn get_routes() -> Scope {
    web::scope("/file/")
        .service(info)
}

#[get("/info/{file_id}")]
async fn info(state: web::Data<State>, file_id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok()
}