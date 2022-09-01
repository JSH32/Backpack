use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::database::entity::registration_keys;

use super::{to_service_result, ServiceResult};

pub struct RegistrationKeyService<'a> {
    database: &'a DatabaseConnection,
}

impl<'a> RegistrationKeyService<'a> {
    pub fn new(database: &'a DatabaseConnection) -> Self {
        Self { database }
    }

    /// Get a registration key by UUID code.
    pub async fn get_registration_key(
        &self,
        code: &str,
    ) -> ServiceResult<registration_keys::Model> {
        to_service_result(
            registration_keys::Entity,
            registration_keys::Entity::find()
                .filter(registration_keys::Column::Code.eq(code))
                .one(self.database)
                .await,
        )
    }
}
