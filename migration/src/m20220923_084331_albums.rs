use sea_orm_migration::{prelude::*, sea_orm::DbBackend};

use crate::extensions::{ColumnExtension, ManagerExtension};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Albums::Table)
                    .col(
                        ColumnDef::new(Albums::Id)
                            .sonyflake()
                            .primary_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Albums::Created)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".into()),
                    )
                    .col(ColumnDef::new(Albums::UserId).sonyflake().not_null())
                    .col(ColumnDef::new(Albums::Name).string_len(16).not_null())
                    .col(ColumnDef::new(Albums::Description).string())
                    .col(
                        ColumnDef::new(Albums::Public)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Albums::Table, Albums::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // No two albums with the same name by the same user.
        manager
            .create_index(
                Index::create()
                    .unique()
                    .name("albums_uindex")
                    .table(Albums::Table)
                    .col(Albums::UserId)
                    .col(Albums::Name)
                    .to_owned(),
            )
            .await?;

        // Add new columns to user.
        if manager.get_database_backend() == DbBackend::Sqlite {
            // SQlite 3.35.0 supports dropping columns but SeaORM hasn't updated yet.
            // TODO: Remove this when SeaORM supports it.
            // https://github.com/SeaQL/sea-orm/issues/1065

            manager
                .exec_sql(
                    r#"
            ALTER TABLE files ADD COLUMN album_id VARCHAR(20);
            ALTER TABLE files ADD COLUMN public boolean NOT NULL DEFAULT false;
            "#,
                )
                .await
        } else {
            manager
                .alter_table(
                    Table::alter()
                        .table(Files::Table)
                        .add_column(ColumnDef::new(Files::AlbumId).sonyflake())
                        .add_column(
                            ColumnDef::new(Files::Public)
                                .boolean()
                                .not_null()
                                .default(false),
                        )
                        .to_owned(),
                )
                .await
        }
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Albums::Table).to_owned())
            .await?;

        if manager.get_database_backend() == DbBackend::Sqlite {
            // SQlite 3.35.0 supports dropping columns but SeaORM hasn't updated yet.
            // TODO: Remove this when SeaORM supports it.
            // https://github.com/SeaQL/sea-orm/issues/1065

            manager
                .exec_sql(
                    r#"
            ALTER TABLE files DROP COLUMN album_id;
            ALTER TABLE files DROP COLUMN public;
            "#,
                )
                .await
        } else {
            manager
                .alter_table(
                    Table::alter()
                        .table(Files::Table)
                        .drop_column(Files::AlbumId)
                        .drop_column(Files::Public)
                        .to_owned(),
                )
                .await
        }
    }
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
}

#[derive(Iden)]
enum Files {
    Table,
    AlbumId,
    Public,
}

#[derive(Iden)]
enum Albums {
    Table,
    Id,
    Created,
    Name,
    Description,
    UserId,
    Public,
}
