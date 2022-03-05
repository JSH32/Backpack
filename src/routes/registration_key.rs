use actix_web::{post, web, Scope, Responder, HttpResponse, http::StatusCode};
use sea_orm::{ActiveModelTrait, ColumnTrait, ModelTrait, QueryFilter, Set};

use crate::{
    database::entity::registration_keys,
    models::{registration_key::*, MessageResponse, Response},
    state::State,
    util::auth::{auth_role, Auth},
};

pub fn get_routes() -> Scope {
    web::scope("/registration_key")
        .service(create)
}

#[post("")]
async fn create(
    state: web::Data<State>,
    auth: Auth<auth_role::Admin, false, true>,
) -> Response<impl Responder> {
    let code_data = RegistrationKeyData::from(
        registration_keys::ActiveModel {
            iss_user: Set(auth.user.id.to_owned()),
            // take input here
            max_uses: Set(1),
            ..Default::default()
        }
        .insert(&state.database)
        .await?
    );
    Ok(HttpResponse::Ok().json(code_data))
}