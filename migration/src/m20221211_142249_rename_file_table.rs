use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .rename_table(
                Table::rename()
                    .table(Alias::new("files"), Alias::new("uploads"))
                    .take(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .rename_table(
                Table::rename()
                    .table(Alias::new("uploads"), Alias::new("files"))
                    .take(),
            )
            .await
    }
}
