use crate::{models::*, state::State};
use crate::util::auth;

use actix_web::*;
use actix_web::http::StatusCode;
use rand::{Rng, distributions::Alphanumeric, rngs::OsRng};

pub fn get_routes() -> Scope {
    web::scope("/token/")
        .service(list)
        .service(info)
        .service(create)
}

#[get("list")]
async fn list(state: web::Data<State>, auth: auth::middleware::User) -> impl Responder {
    match state.database.get_all_tokens(auth.0.id).await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(_) => MessageResponse::internal_server_error().http_response()
    }
}

#[get("info")]
async fn info(state: web::Data<State>, auth: auth::middleware::User) -> impl Responder {
    HttpResponse::new(StatusCode::NOT_IMPLEMENTED)
}

#[post("create")]
async fn create(state: web::Data<State>, auth: auth::middleware::User, form: web::Json<TokenCreateForm>) -> impl Responder {
    // Check if token count is over 10
    match state.database.get_token_count(auth.0.id).await {
        Ok(count) => {
            if count > 10 {
                return MessageResponse::new(StatusCode::BAD_REQUEST, "The token limit per user is 10").http_response()
            }
        },
        Err(_) => return MessageResponse::internal_server_error().http_response()
    };

    // Get underlying non wrapped data struct, allows borrows
    let data = form.into_inner();

    if data.name.len() > 32 {
        return MessageResponse::new(StatusCode::BAD_REQUEST, "Token name too long (maximum 32 characters)").http_response()
    } else if data.name.len() < 4 {
        return MessageResponse::new(StatusCode::BAD_REQUEST, "Token name too short (maximum 4 characters)").http_response()
    }

    if let Some(desc) = &data.description {
        if desc.len() > 512 {
            return MessageResponse::new(StatusCode::BAD_REQUEST, "Token description too long (maximum 512 characters)").http_response()
        } else if desc.len() == 0 {
            return MessageResponse::new(StatusCode::BAD_REQUEST, "Description must not be empty, can be null").http_response()
        }
    }

    match state.database.check_token_exist(auth.0.id, &data.name).await {
        Err(_) => return MessageResponse::internal_server_error().http_response(),
        Ok(val) => {
            if val {
                return MessageResponse::new(StatusCode::BAD_REQUEST, "A token with that name already exists").http_response();
            }
        }
    }

    let token: String = OsRng.sample_iter(&Alphanumeric).take(128).map(char::from).collect();

    match state.database.create_token(auth.0.id, &data.name, &data.description, &token).await {
        Ok(new_token) => HttpResponse::Ok().json(new_token),
        Err(_) =>  MessageResponse::internal_server_error().http_response()
    }
}

// #[get("delete")]
// async fn delete(state: web::Data<State>, auth: auth::middleware::User, data: web::Json<IDQuery>) -> impl Responder {
//     HttpResponse::new(StatusCode::NOT_IMPLEMENTED)
// }