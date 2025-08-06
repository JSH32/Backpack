use sea_orm_migration::prelude::*;

use crate::extensions::{ColumnExtension, ManagerExtension};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_fkey(Uploads::Table, Uploads::AlbumId, "files_album_id_fkey")
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Uploads::Table)
                    .drop_column(Uploads::AlbumId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AlbumUploads::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AlbumUploads::Id)
                            .sonyflake()
                            .primary_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AlbumUploads::Created)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(AlbumUploads::UploadId)
                            .sonyflake()
                            .not_null(),
                    )
                    .col(ColumnDef::new(AlbumUploads::AlbumId).sonyflake().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_album_uploads_album_id")
                            .from(AlbumUploads::Table, AlbumUploads::AlbumId)
                            .to(Albums::Table, Albums::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_album_uploads_upload_id")
                            .from(AlbumUploads::Table, AlbumUploads::UploadId)
                            .to(Uploads::Table, Uploads::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Uploads::Table)
                    .add_column(ColumnDef::new(Uploads::AlbumId).sonyflake())
                    .to_owned(),
            )
            .await?;

        manager
            .create_fkey(
                Uploads::Table,
                ForeignKey::create()
                    .name("files_album_id_fkey")
                    .from(Uploads::Table, Uploads::AlbumId)
                    .to(Albums::Table, Albums::Id)
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum AlbumUploads {
    Table,
    Id,
    Created,
    AlbumId,
    UploadId,
}

#[derive(DeriveIden)]
enum Albums {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Uploads {
    Table,
    Id,
    AlbumId,
}
