use sea_orm_migration::{prelude::*, sea_orm::DbBackend, sea_query::extension::postgres::Type};

use crate::extensions::{ColumnExtension, ManagerExtension};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if manager.get_database_backend() == DbBackend::Postgres {
            manager
                .create_type(
                    Type::create()
                        .as_enum(AuthMethod::Type)
                        .values(vec![
                            AuthMethod::Password,
                            AuthMethod::Google,
                            AuthMethod::Github,
                            AuthMethod::Discord,
                        ])
                        .to_owned(),
                )
                .await?;
        }

        manager
            .create_table(
                Table::create()
                    .table(AuthMethods::Table)
                    .col(
                        ColumnDef::new(AuthMethods::Id)
                            .sonyflake()
                            .primary_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(AuthMethods::UserId).sonyflake().not_null())
                    .col(
                        ColumnDef::new(AuthMethods::AuthMethod)
                            .enumeration("auth_method", ["password", "google", "github", "discord"])
                            .not_null(),
                    )
                    .col(ColumnDef::new(AuthMethods::CachedUsername).text())
                    .col(ColumnDef::new(AuthMethods::Value).text().not_null())
                    .col(
                        ColumnDef::new(AuthMethods::LastAccessed)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".into()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(AuthMethods::Table, AuthMethods::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // A user may not have two of the same type of method.
        manager
            .create_index(
                Index::create()
                    .unique()
                    .name("auth_methods_uindex")
                    .table(AuthMethods::Table)
                    .col(AuthMethods::UserId)
                    .col(AuthMethods::AuthMethod)
                    .to_owned(),
            )
            .await?;

        // Auth methods in another table including password.
        manager.drop_column(Users::Table, Users::Password).await?;

        // Post registration can occur, add registered column.
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .add_column(ColumnDef::new(Users::Registered).boolean().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AuthMethods::Table).to_owned())
            .await?;

        if manager.get_database_backend() == DbBackend::Postgres {
            manager
                .drop_type(Type::drop().name(AuthMethod::Type).to_owned())
                .await?;
        }

        // Post registration can occur, add registered column.
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .add_column(ColumnDef::new(Users::Password).string_len(128).not_null())
                    .to_owned(),
            )
            .await?;

        manager.drop_column(Users::Table, Users::Registered).await
    }
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
    Password,
    Registered,
}

#[derive(Iden)]
enum AuthMethods {
    Table,
    Id,
    UserId,
    CachedUsername,
    AuthMethod,
    Value,
    LastAccessed,
}

#[derive(Iden)]
enum AuthMethod {
    #[iden = "auth_method"]
    Type,
    Password,
    Google,
    Github,
    Discord,
}
