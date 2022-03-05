use serde::Serialize;
use crate::database::entity::registration_keys;
use sea_orm::prelude::{Uuid, DateTimeUtc};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationKeyData {
    pub id: String,
    pub code: Uuid,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<DateTimeUtc>,
    pub iss_user: String,
    pub used: i32,
    pub max_uses: i32,
}

impl From<registration_keys::Model> for RegistrationKeyData {
    fn from(model: registration_keys::Model) -> Self {
        Self {
            id: model.id.to_string(),
            code: model.code,
            expiry_date: model.expiry_date.into(),
            iss_user: model.iss_user,
            used: model.used,
            max_uses: model.max_uses,
        }
    }
}

pub struct RegisterKeyCreateData {
    pub max_uses: i32,
    // we need to take expires in.
}