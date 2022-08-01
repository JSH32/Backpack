use crate::database::entity::registration_keys;
use sea_orm::prelude::{DateTimeUtc, Uuid};
use serde::{Deserialize, Serialize};
use utoipa::{Component, IntoParams};

#[derive(Serialize, Component)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationKeyData {
    pub id: String,

    /// Admin which issued this registration key.
    pub iss_user: String,

    /// Registration key.
    #[component(value_type = String)]
    pub code: Uuid,

    /// Amount of uses left.
    pub uses_left: i32,

    /// Key invalidation date.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[component(value_type = String)]
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

#[derive(Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationKeyParams {
    /// Maximum amount of key uses.
    pub max_uses: Option<i32>,
}
