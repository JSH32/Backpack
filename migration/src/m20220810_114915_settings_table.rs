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
                        .as_enum(ThemeColor::Type)
                        .values(vec![
                            ThemeColor::Gray,
                            ThemeColor::Red,
                            ThemeColor::Orange,
                            ThemeColor::Yellow,
                            ThemeColor::Green,
                            ThemeColor::Teal,
                            ThemeColor::Blue,
                            ThemeColor::Cyan,
                            ThemeColor::Purple,
                            ThemeColor::Pink,
                        ])
                        .to_owned(),
                )
                .await?;
        }

        manager
            .create_table(
                Table::create()
                    .table(Settings::Table)
                    .col(
                        ColumnDef::new(Settings::OneRowEnforce)
                            .primary_key()
                            .not_null()
                            .boolean()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(Settings::AppName)
                            .string_len(64)
                            .not_null()
                            .default("Backpack"),
                    )
                    .col(
                        ColumnDef::new(Settings::AppDescription)
                            .text()
                            .not_null()
                            .default("A file host for all your needs"),
                    )
                    .col(
                        ColumnDef::new(Settings::Color)
                            .enumeration(
                                "theme_color",
                                [
                                    "gray", "red", "orange", "yellow", "green", "teal", "blue",
                                    "cyan", "purple", "pink",
                                ],
                            )
                            .default("purple")
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Insert default settings at first, configuration done through web UI
        manager
            .exec_stmt(
                Query::insert()
                    .into_table(Settings::Table)
                    .or_default_values()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Settings::Table).to_owned())
            .await?;

        if manager.get_database_backend() == DbBackend::Postgres {
            manager
                .drop_type(Type::drop().name(ThemeColor::Type).to_owned())
                .await?;
        }

        Ok(())
    }
}

#[derive(Iden)]
enum Settings {
    Table,
    OneRowEnforce,
    AppName,
    AppDescription,
    Color,
}

#[derive(Iden)]
enum ThemeColor {
    #[iden = "theme_color"]
    Type,
    Gray,
    Red,
    Orange,
    Yellow,
    Green,
    Teal,
    Blue,
    Cyan,
    Purple,
    Pink,
}
