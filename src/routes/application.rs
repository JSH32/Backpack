use actix_web::{delete, get, http::StatusCode, post, web, HttpResponse, Responder, Scope};
use sea_orm::{ActiveModelTrait, ColumnTrait, ModelTrait, PaginatorTrait, QueryFilter, Set};

use crate::{
    database::entity::applications,
    internal::{
        auth::{auth_role, create_jwt_string, Auth},
        response::Response,
    },
    models::{application::*, MessageResponse},
    state::State,
};

pub fn get_routes() -> Scope {
    web::scope("/application")
        .service(list)
        .service(info)
        .service(create)
        .service(delete)
        .service(token)
}

/// Get token by application ID
/// - Minimum required role: `user`
/// - Allow unverified users: `false`
/// - Application token allowed: `false`
#[utoipa::path(
    context_path = "/api/application",
    tag = "application",
    responses(
        (status = 200, body = TokenResponse),
        (status = 404, body = MessageResponse, description = "Application not found")
    ),
    params(
        ("application_id" = str, path, description = "Application ID to get token for"),
    ),
    security(("apiKey" = [])),
)]
#[get("/{application_id}/token")]
async fn token(
    state: web::Data<State>,
    application_id: web::Path<String>,
    user: Auth<auth_role::User>,
) -> Response<impl Responder> {
    Ok(
        match user
            .find_related(applications::Entity)
            .filter(applications::Column::Id.eq(application_id.to_string()))
            .one(&state.database)
            .await?
        {
            Some(v) => HttpResponse::Ok().json(TokenResponse {
                token: create_jwt_string(
                    &user.id,
                    Some(v.id),
                    &state
                        .api_url
                        .host()
                        .expect("API_URL must have host included"),
                    None,
                    &state.jwt_key,
                )?,
            }),
            None => MessageResponse::new(StatusCode::NOT_FOUND, "That application was not found")
                .http_response(),
        },
    )
}

/// Get all applications
/// - Minimum required role: `user`
/// - Allow unverified users: `false`
/// - Application token allowed: `false`
#[utoipa::path(
    context_path = "/api/application",
    tag = "application",
    responses((status = 200, body = [ApplicationData])),
    security(("apiKey" = [])),
)]
#[get("")]
async fn list(state: web::Data<State>, user: Auth<auth_role::User>) -> Response<impl Responder> {
    let applications: Vec<ApplicationData> = user
        .find_related(applications::Entity)
        .all(&state.database)
        .await?
        .iter()
        .map(|e| ApplicationData::from(e.to_owned()))
        .collect();

    Ok(HttpResponse::Ok().json(applications))
}

/// Get token info
/// - Minimum required role: `user`
/// - Allow unverified users: `false`
/// - Application token allowed: `false`
#[utoipa::path(
    context_path = "/api/application",
    tag = "application",
    responses(
        (status = 200, body = ApplicationData),
        (status = 401, body = MessageResponse)
    ),
    security(("apiKey" = [])),
)]
#[get("/{application_id}")]
async fn info(
    state: web::Data<State>,
    user: Auth<auth_role::User>,
    application_id: web::Path<String>,
) -> Response<impl Responder> {
    Ok(
        match user
            .find_related(applications::Entity)
            .filter(applications::Column::Id.eq(application_id.as_str()))
            .one(&state.database)
            .await?
        {
            Some(data) => HttpResponse::Ok().json(ApplicationData::from(data)),
            None => MessageResponse::unauthorized_error().http_response(),
        },
    )
}

/// Create an application
/// - Minimum required role: `user`
/// - Allow unverified users: `false`
/// - Application token allowed: `false`
#[utoipa::path(
    context_path = "/api/application",
    tag = "application",
    responses(
        (status = 200, body = ApplicationData),
        (status = 400, body = MessageResponse, description = "Token limit reached or invalid name"),
    ),
    request_body = ApplicationCreate,
    security(("apiKey" = [])),
)]
#[post("")]
async fn create(
    state: web::Data<State>,
    user: Auth<auth_role::User>,
    form: web::Json<ApplicationCreate>,
) -> Response<impl Responder> {
    let application_count = user
        .find_related(applications::Entity)
        .count(&state.database)
        .await?;

    if application_count >= 5 {
        return MessageResponse::ok(StatusCode::BAD_REQUEST, "The token limit per user is 5");
    }

    if (&form).name.len() > 16 {
        return MessageResponse::ok(
            StatusCode::BAD_REQUEST,
            "Token name too long (maximum 16 characters)",
        );
    } else if (&form).name.len() < 4 {
        return MessageResponse::ok(
            StatusCode::BAD_REQUEST,
            "Token name too short (minimum 4 characters)",
        );
    }

    if let Some(_) = user
        .find_related(applications::Entity)
        .filter(applications::Column::Name.eq(form.name.to_owned()))
        .one(&state.database)
        .await?
    {
        return MessageResponse::ok(
            StatusCode::BAD_REQUEST,
            "A token with that name already exists",
        );
    }

    // Create an application token and send JWT to user
    let mut token_data = ApplicationData::from(
        applications::ActiveModel {
            user_id: Set(user.id.to_owned()),
            name: Set(form.name.to_owned()),
            ..Default::default()
        }
        .insert(&state.database)
        .await?,
    );

    token_data.token = Some(create_jwt_string(
        &user.id,
        Some(token_data.id.clone()),
        &state
            .api_url
            .host()
            .expect("API_URL must have host included"),
        None,
        &state.jwt_key,
    )?);

    Ok(HttpResponse::Ok().json(token_data))
}

/// Delete an application
/// - Minimum required role: `user`
/// - Allow unverified users: `false`
/// - Application token allowed: `false`
#[utoipa::path(
    context_path = "/api/application",
    tag = "application",
    responses(
        (status = 200, body = MessageResponse, description = "Application was deleted"),
        (status = 401, body = MessageResponse, description = "Unauthorized or token does not exist"),
    ),
    security(("apiKey" = [])),
)]
#[delete("/{application_id}")]
async fn delete(
    state: web::Data<State>,
    user: Auth<auth_role::User>,
    application_id: web::Path<String>,
) -> Response<impl Responder> {
    Ok(
        match user
            .find_related(applications::Entity)
            .filter(applications::Column::Id.eq(application_id.to_string()))
            .one(&state.database)
            .await?
        {
            Some(v) => {
                v.delete(&state.database).await?;
                MessageResponse::new(StatusCode::OK, "Application was successfully deleted")
            }
            None => MessageResponse::unauthorized_error(),
        },
    )
}
