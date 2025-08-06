pub use sea_orm_migration::prelude::*;

mod extensions;
mod m20220101_000001_initial_structure;
mod m20220810_114915_settings_table;
mod m20220920_105037_auth_methods;
mod m20220923_084331_albums;
mod m20221211_142249_rename_file_table;
mod m20250806_193722_multi_owner_album;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_initial_structure::Migration),
            Box::new(m20220810_114915_settings_table::Migration),
            Box::new(m20220920_105037_auth_methods::Migration),
            Box::new(m20220923_084331_albums::Migration),
            Box::new(m20221211_142249_rename_file_table::Migration),
            Box::new(m20250806_193722_multi_owner_album::Migration),
        ]
    }
}
