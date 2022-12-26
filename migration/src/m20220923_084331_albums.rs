use sea_orm_migration::prelude::*;

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
                            .name("files_album_id_fkey")
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

        manager
            .alter_table(
                Table::alter()
                    .table(Files::Table)
                    .add_column(ColumnDef::new(Files::AlbumId).sonyflake())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Files::Table)
                    .add_column(
                        ColumnDef::new(Files::Public)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_fkey(
                Files::Table,
                ForeignKey::create()
                    .name("files_album_id_fkey")
                    .from(Files::Table, Files::AlbumId)
                    .to(Albums::Table, Albums::Id)
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_fkey(Files::Table, Files::AlbumId, "files_album_id_fkey")
            .await?;

        manager.drop_column(Files::Table, Files::AlbumId).await?;
        manager.drop_column(Files::Table, Files::Public).await?;

        manager
            .drop_table(Table::drop().table(Albums::Table).to_owned())
            .await
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
