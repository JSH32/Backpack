use crate::database::entity::registration_keys;
use sea_orm::prelude::{DateTimeUtc, Uuid};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationKeyData {
    pub id: String,
    pub iss_user: String,
    // Since admin is the only one who can access this
    // There is no point in hiding it
    pub code: Uuid,
    pub uses_left: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<DateTimeUtc>,
}

impl From<registration_keys::Model> for RegistrationKeyData {
    fn from(model: registration_keys::Model) -> Self {
        Self {
            id: model.id.to_string(),
            code: model.code,
            expiry_date: model.expiry_date,
            iss_user: model.iss_user,
            uses_left: model.uses_left,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationKeyParams {
    pub max_uses: Option<i32>,
}
