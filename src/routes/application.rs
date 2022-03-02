use actix_web::{delete, get, http::StatusCode, post, web, HttpResponse, Responder, Scope};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ModelTrait, PaginatorTrait, QueryFilter, Set,
};

use crate::{
    database::entity::applications,
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
    Ok(
        match auth.user
            .find_related(applications::Entity)
            .filter(applications::Column::Id.eq(application_id.to_string()))
            .one(&state.database)
            .await?
        {
            Some(v) => HttpResponse::Ok().json(TokenResponse {
                token: create_jwt_string(
                    &auth.user.id,
                    Some(v.id),
                    &state
                        .base_url
                        .host()
                        .expect("BASE_URL must have host included"),
                    None,
                    &state.jwt_key,
                )?,
            }),
            None => MessageResponse::new(StatusCode::NOT_FOUND, "That application was not found")
                .http_response(),
        },
    )
}

#[get("")]
async fn list(
    state: web::Data<State>,
    auth: Auth<auth_role::User, false, false>,
) -> Response<impl Responder> {
    let applications: Vec<ApplicationData> = auth
        .user
        .find_related(applications::Entity)
        .all(&state.database)
        .await?
        .iter()
        .map(|e| ApplicationData::from(e.to_owned()))
        .collect();

    Ok(HttpResponse::Ok().json(applications))
}

#[get("/{application_id}")]
async fn info(
    state: web::Data<State>,
    auth: Auth<auth_role::User, false, false>,
    application_id: web::Path<String>,
) -> Response<impl Responder> {
    Ok(
        match auth
            .user
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

#[post("")]
async fn create(
    state: web::Data<State>,
    auth: Auth<auth_role::User, false, false>,
    form: web::Json<ApplicationCreateForm>,
) -> Response<impl Responder> {
    let application_count = auth
        .user
        .find_related(applications::Entity)
        .count(&state.database)
        .await?;

    if application_count >= 5 {
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
            "Token name too short (minimum 4 characters)",
        )
        .http_response());
    }

    if let Some(_) = auth
        .user
        .find_related(applications::Entity)
        .filter(applications::Column::Name.eq(form.name.to_owned()))
        .one(&state.database)
        .await?
    {
        return Ok(MessageResponse::new(
            StatusCode::BAD_REQUEST,
            "A token with that name already exists",
        )
        .http_response());
    }

    // Create an application token and send JWT to user
    let mut token_data = ApplicationData::from(
        applications::ActiveModel {
            user_id: Set(auth.user.id.to_owned()),
            name: Set(form.name.to_owned()),
            ..Default::default()
        }
        .insert(&state.database)
        .await?,
    );

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
    Ok(
        match auth.user
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
