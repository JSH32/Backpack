use actix_web::{delete, get, http::StatusCode, post, web, HttpResponse, Responder, Scope};

use crate::{
    models::{application::*, MessageResponse, Response},
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
) -> Response<impl Responder> {
    Ok(HttpResponse::Ok().json(TokenResponse {
        token: create_jwt_string(
            &auth.user.id,
            Some(
                state
                    .database
                    .get_application_by_id(&application_id)
                    .await?
                    .id,
            ),
            &state
                .base_url
                .host()
                .expect("BASE_URL must have host included"),
            None,
            &state.jwt_key,
        )?,
    }))
}

#[get("")]
async fn list(
    state: web::Data<State>,
    auth: Auth<auth_role::User, false, false>,
) -> Response<impl Responder> {
    Ok(HttpResponse::Ok().json(state.database.get_all_applications(&auth.user.id).await?))
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
) -> Response<impl Responder> {
    // Check if application count is over 5
    let application_count = state.database.application_count(&auth.user.id).await?;
    if application_count > 5 {
        return Ok(
            MessageResponse::new(StatusCode::BAD_REQUEST, "The token limit per user is 5")
                .http_response(),
        );
    }

    if (&form).name.len() > 32 {
        return Ok(MessageResponse::new(
            StatusCode::BAD_REQUEST,
            "Token name too long (maximum 32 characters)",
        )
        .http_response());
    } else if (&form).name.len() < 4 {
        return Ok(MessageResponse::new(
            StatusCode::BAD_REQUEST,
            "Token name too short (maximum 4 characters)",
        )
        .http_response());
    }

    if state
        .database
        .application_exist(&auth.user.id, &form.name)
        .await?
    {
        return Ok(MessageResponse::new(
            StatusCode::BAD_REQUEST,
            "A token with that name already exists",
        )
        .http_response());
    }

    // Create an application token and send JWT to user
    let mut token_data = state
        .database
        .create_application(&auth.user.id, &form.name)
        .await?;

    token_data.token = Some(create_jwt_string(
        &auth.user.id,
        Some(token_data.id.clone()),
        &state
            .base_url
            .host()
            .expect("BASE_URL must have host included"),
        None,
        &state.jwt_key,
    )?);

    Ok(HttpResponse::Ok().json(token_data))
}

#[delete("/{application_id}")]
async fn delete(
    state: web::Data<State>,
    auth: Auth<auth_role::User, false, false>,
    application_id: web::Path<String>,
) -> Response<impl Responder> {
    let application_id = match state.database.get_application_by_id(&application_id).await {
        Ok(application_data) => {
            if application_data.user_id != auth.user.id {
                return Ok(MessageResponse::unauthorized_error());
            }
            application_data.id
        }
        Err(_) => return Ok(MessageResponse::unauthorized_error()),
    };

    state
        .database
        .delete_application_by_id(&application_id)
        .await?;

    Ok(MessageResponse::new(
        StatusCode::OK,
        "Application was successfully deleted",
    ))
}
