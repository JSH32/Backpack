use crate::{models::{MessageResponse, token::{TokenCreateForm, TokenQuery}}, state::State, util::auth::create_jwt_string};
use crate::util::auth;

use actix_web::*;
use actix_web::http::StatusCode;

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
async fn info(state: web::Data<State>, auth: auth::middleware::User, info: web::Query<TokenQuery>) -> impl Responder {
    match state.database.get_token_by_id(info.id).await {
        Ok(data) => {
            if data.user_id != auth.0.id {
                return MessageResponse::unauthorized_error().http_response();
            }
            return HttpResponse::Ok().json(data);
        },
        // Return unauthorized so people cannot see which IDs exist and which don't
        Err(_) => return MessageResponse::unauthorized_error().http_response()
    }
}

#[post("create")]
async fn create(state: web::Data<State>, auth: auth::middleware::User, form: web::Json<TokenCreateForm>) -> impl Responder {
    // Check if token count is over 10
    match state.database.token_count(auth.0.id).await {
        Ok(count) => {
            if count > 5 {
                return MessageResponse::new(StatusCode::BAD_REQUEST, "The token limit per user is 5").http_response()
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

    match state.database.token_exist(auth.0.id, &data.name).await {
        Err(_) => return MessageResponse::internal_server_error().http_response(),
        Ok(val) => {
            if val {
                return MessageResponse::new(StatusCode::BAD_REQUEST, "A token with that name already exists").http_response();
            }
        }
    }

    // Create a perm token and send JWT to user
    match state.database.create_token(auth.0.id, &data.name).await {
        Err(_) => return MessageResponse::internal_server_error().http_response(),
        Ok(mut token) => {
            // Add the token to a JSON field on creation, will only be showed once
            match create_jwt_string(auth.0.id, Some(token.id), "localhost", None, &state.jwt_key) {
                Err(_) => return MessageResponse::internal_server_error().http_response(),
                Ok(jwt_string) => {
                    token.token = Some(jwt_string);
                    HttpResponse::Ok().json(token)
                }
            }
        }
    }
}

// #[get("delete")]
// async fn delete(state: web::Data<State>, auth: auth::middleware::User, data: web::Json<IDQuery>) -> impl Responder {
//     HttpResponse::new(StatusCode::NOT_IMPLEMENTED)
// }