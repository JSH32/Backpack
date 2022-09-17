use crate::extensions::*;
use sea_orm_migration::{prelude::*, sea_orm::DbBackend, sea_query::extension::postgres::Type};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the actual enum types if using postgres.
        if manager.get_database_backend() == DbBackend::Postgres {
            manager
                .create_type(
                    Type::create()
                        .as_enum(Role::Type)
                        .values(vec![Role::User, Role::Admin])
                        .to_owned(),
                )
                .await?;
        }

        // User table
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .col(
                        ColumnDef::new(Users::Id)
                            .sonyflake()
                            .primary_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::Email)
                            .string_len(320)
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::Username)
                            .string_len(32)
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Users::Password).string_len(128).not_null())
                    .col(
                        ColumnDef::new(Users::Verified)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::Role)
                            .enumeration("role", ["user", "admin"])
                            .default("user")
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Applications table
        manager
            .create_table(
                Table::create()
                    .table(Applications::Table)
                    .col(
                        ColumnDef::new(Applications::Id)
                            .sonyflake()
                            .primary_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Applications::UserId).string().not_null())
                    .col(ColumnDef::new(Applications::Name).string_len(16).not_null())
                    .col(
                        ColumnDef::new(Applications::LastAccessed)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".into()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Applications::Table, Applications::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // A user may not have two duplicate application names
        manager
            .create_index(
                Index::create()
                    .unique()
                    .name("applications_name_uindex")
                    .table(Applications::Table)
                    .col(Applications::UserId)
                    .col(Applications::Name)
                    .to_owned(),
            )
            .await?;

        // User verification
        // Only one verification may exist per user
        manager
            .create_table(
                Table::create()
                    .table(Verifications::Table)
                    .col(
                        ColumnDef::new(Verifications::Id)
                            .sonyflake()
                            .primary_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Verifications::Code)
                            .string_len(72)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Verifications::UserId)
                            .sonyflake()
                            .not_null()
                            .unique_key(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Verifications::Table, Verifications::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // File table
        manager
            .create_table(
                Table::create()
                    .table(Files::Table)
                    .col(
                        ColumnDef::new(Files::Id)
                            .sonyflake()
                            .primary_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Files::Name)
                            .string_len(32)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Files::OriginalName)
                            .string_len(256)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Files::Uploader).sonyflake().not_null())
                    .col(ColumnDef::new(Files::Hash).string_len(64).not_null())
                    .col(
                        ColumnDef::new(Files::Uploaded)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".into()),
                    )
                    .col(ColumnDef::new(Files::Size).big_integer().not_null())
                    .col(
                        ColumnDef::new(Files::HasThumbnail)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Files::Table, Files::Uploader)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Two identical files by hash can not exist if owned by the same user
        manager
            .create_index(
                Index::create()
                    .unique()
                    .name("files_user_hash_uindex")
                    .table(Files::Table)
                    .col(Files::Uploader)
                    .col(Files::Hash)
                    .to_owned(),
            )
            .await?;

        // User registration keys
        manager
            .create_table(
                Table::create()
                    .table(RegistrationKeys::Table)
                    .col(
                        ColumnDef::new(RegistrationKeys::Id)
                            .sonyflake()
                            .primary_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RegistrationKeys::Issuer)
                            .sonyflake()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RegistrationKeys::Code)
                            .uuid()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(RegistrationKeys::UsesLeft).integer())
                    .col(ColumnDef::new(RegistrationKeys::ExpiryDate).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Applications::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Verifications::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Files::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(RegistrationKeys::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;

        if manager.get_database_backend() == DbBackend::Postgres {
            manager
                .drop_type(Type::drop().name(Role::Type).to_owned())
                .await?;
        }

        Ok(())
    }
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
    Email,
    Username,
    Password,
    Verified,
    Role,
}

#[derive(Iden)]
enum Applications {
    Table,
    Id,
    UserId,
    Name,
    LastAccessed,
}

#[derive(Iden)]
enum Verifications {
    Table,
    Id,
    Code,
    UserId,
}

#[derive(Iden)]
enum Files {
    Table,
    Id,
    Name,
    OriginalName,
    HasThumbnail,
    Uploader,
    Hash,
    Uploaded,
    Size,
}

#[derive(Iden)]
enum RegistrationKeys {
    Table,
    Id,
    Issuer,
    Code,
    UsesLeft,
    ExpiryDate,
}

#[derive(Iden)]
enum Role {
    #[iden = "role"]
    Type,
    User,
    Admin,
}
