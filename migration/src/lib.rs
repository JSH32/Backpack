pub use sea_orm_migration::prelude::*;

mod extensions;
mod m20220101_000001_initial_structure;
mod m20220810_114915_settings_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_initial_structure::Migration),
            Box::new(m20220810_114915_settings_table::Migration),
        ]
    }
}
