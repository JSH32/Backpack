use actix_web::{delete, get, http::StatusCode, post, web, HttpResponse, Responder, Scope};

use crate::{
    models::{application::*, MessageResponse},
    state::State,
    util::auth::{auth_role, create_jwt_string, Auth},
};

pub fn get_routes() -> Scope {
    web::scope("/applications")
        .service(list)
        .service(info)
        .service(create)
        .service(delete)
        .service(token)
}

#[get("/{application_id}/token")]
async fn token(
    state: web::Data<State>,
    application_id: web::Path<String>,
    auth: Auth<auth_role::User, false, false>,
) -> impl Responder {
    match state.database.get_application_by_id(&application_id).await {
        Ok(token) => {
            // Add the token to a JSON field on creation, will only be showed once
            match create_jwt_string(
                &auth.user.id,
                Some(token.id.clone()),
                &state
                    .base_url
                    .host()
                    .expect("BASE_URL must have host included"),
                None,
                &state.jwt_key,
            ) {
                Ok(jwt_string) => HttpResponse::Ok().json(TokenResponse { token: jwt_string }),
                Err(_) => return MessageResponse::internal_server_error().http_response(),
            }
        }
        Err(_) => MessageResponse::internal_server_error().http_response(),
    }
}

#[get("")]
async fn list(
    state: web::Data<State>,
    auth: Auth<auth_role::User, false, false>,
) -> impl Responder {
    match state.database.get_all_applications(&auth.user.id).await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(_) => MessageResponse::internal_server_error().http_response(),
    }
}

#[get("/{application_id}")]
async fn info(
    state: web::Data<State>,
    auth: Auth<auth_role::User, false, false>,
    application_id: web::Path<String>,
) -> impl Responder {
    match state.database.get_application_by_id(&application_id).await {
        Ok(data) => {
            if data.user_id != auth.user.id {
                return MessageResponse::unauthorized_error().http_response();
            }
            return HttpResponse::Ok().json(data);
        }
        // Return unauthorized so people cannot see which IDs exist and which don't
        Err(_) => return MessageResponse::unauthorized_error().http_response(),
    }
}

#[post("")]
async fn create(
    state: web::Data<State>,
    auth: Auth<auth_role::User, false, false>,
    form: web::Json<ApplicationCreateForm>,
) -> impl Responder {
    // Check if application count is over 5
    match state.database.application_count(&auth.user.id).await {
        Ok(count) => {
            if count > 5 {
                return MessageResponse::new(
                    StatusCode::BAD_REQUEST,
                    "The token limit per user is 5",
                )
                .http_response();
            }
        }
        Err(_) => return MessageResponse::internal_server_error().http_response(),
    };

    if (&form).name.len() > 32 {
        return MessageResponse::new(
            StatusCode::BAD_REQUEST,
            "Token name too long (maximum 32 characters)",
        )
        .http_response();
    } else if (&form).name.len() < 4 {
        return MessageResponse::new(
            StatusCode::BAD_REQUEST,
            "Token name too short (maximum 4 characters)",
        )
        .http_response();
    }

    match state
        .database
        .application_exist(&auth.user.id, &form.name)
        .await
    {
        Err(_) => return MessageResponse::internal_server_error().http_response(),
        Ok(val) => {
            if val {
                return MessageResponse::new(
                    StatusCode::BAD_REQUEST,
                    "A token with that name already exists",
                )
                .http_response();
            }
        }
    }

    // Create an application token and send JWT to user
    match state
        .database
        .create_application(&auth.user.id, &form.name)
        .await
    {
        Err(_) => return MessageResponse::internal_server_error().http_response(),
        Ok(mut token_data) => {
            // Add the token to a JSON field on creation, will only be showed once
            match create_jwt_string(
                &auth.user.id,
                Some(token_data.id.clone()),
                &state
                    .base_url
                    .host()
                    .expect("BASE_URL must have host included"),
                None,
                &state.jwt_key,
            ) {
                Err(_) => return MessageResponse::internal_server_error().http_response(),
                Ok(jwt_string) => {
                    token_data.token = Some(jwt_string);
                    HttpResponse::Ok().json(token_data)
                }
            }
        }
    }
}

#[delete("/{application_id}")]
async fn delete(
    state: web::Data<State>,
    auth: Auth<auth_role::User, false, false>,
    application_id: web::Path<String>,
) -> impl Responder {
    let application_id = match state.database.get_application_by_id(&application_id).await {
        Ok(application_data) => {
            if application_data.user_id != auth.user.id {
                return MessageResponse::unauthorized_error();
            }
            application_data.id
        }
        Err(_) => return MessageResponse::unauthorized_error(),
    };

    if let Err(_) = state
        .database
        .delete_application_by_id(&application_id)
        .await
    {
        return MessageResponse::internal_server_error();
    }

    MessageResponse::new(StatusCode::OK, "Application was successfully deleted")
}
