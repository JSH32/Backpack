use crate::database::entity::registration_keys;
use sea_orm::prelude::{DateTimeUtc, Uuid};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationKeyData {
    pub id: String,

    /// Admin which issued this registration key.
    pub issuer: String,

    /// Registration key.
    #[schema(value_type = String)]
    pub code: Uuid,

    /// Amount of uses left.
    pub uses_left: Option<i32>,

    /// Key invalidation date.
    #[schema(value_type = String)]
    pub expiry_date: Option<DateTimeUtc>,
}

impl From<registration_keys::Model> for RegistrationKeyData {
    fn from(model: registration_keys::Model) -> Self {
        Self {
            id: model.id.to_string(),
            code: model.code,
            expiry_date: model.expiry_date,
            issuer: model.issuer,
            uses_left: model.uses_left,
        }
    }
}

#[derive(Deserialize, IntoParams)]
pub struct RegistrationKeyParams {
    /// Maximum amount of key uses.
    pub uses: Option<i32>,
    /// Expiration in milliseconds from creation date.
    pub expiration: Option<i64>,
}
