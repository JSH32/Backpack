use sea_orm_migration::{
    async_trait::async_trait,
    sea_orm::{ConnectionTrait, DbBackend, Statement},
    sea_query::{
        ColumnDef, ForeignKey, ForeignKeyBuilder, ForeignKeyCreateStatement, Iden, Mode, SqlWriter,
        SqliteQueryBuilder, Table,
    },
    DbErr, SchemaManager,
};

/// Adds extra implementation utilities for migrations.
pub trait ColumnExtension {
    fn sonyflake(&mut self) -> &mut Self;
}

impl ColumnExtension for ColumnDef {
    /// Alias for `string_len(20)`, a sonyflake is 20 chars long.
    fn sonyflake(&mut self) -> &mut Self {
        self.string_len(20)
    }
}

#[async_trait]
pub trait ManagerExtension {
    async fn exec_sql(&self, sql: &str) -> Result<(), DbErr>;

    /// Get sqlite table definition as a string in SQlite.
    async fn sqlite_get_table_def(&self, tbl: &str) -> Result<String, DbErr>;

    /// Alternative [`SchemaManager::create_foreign_key`] that supports SQlite.
    /// This is more dangerous and can mess up your schema due to overwriting the master in SQlite.
    /// Foreign keys will not be named in SQlite.
    async fn create_fkey<T: Iden + 'static>(
        &self,
        table: T,
        stmt: ForeignKeyCreateStatement,
    ) -> Result<(), DbErr>;

    /// Alternative [`SchemaManager::drop_foreign_key`] that supports SQlite.
    /// This is more dangerous and can mess up your schema due to overwriting the master in SQlite.
    /// `fkey_name` will be used for all databases except SQlite which will use `column` since fkeys aren't named in SQlite.
    async fn drop_fkey<T: Iden + 'static, C: Iden + 'static>(
        &self,
        table: T,
        column: C,
        fkey_name: &str,
    ) -> Result<(), DbErr>;

    /// Drop a column on any database including SQlite.
    /// This can be removed after this is natively supported:
    /// https://github.com/SeaQL/sea-query/issues/457
    async fn drop_column<T: Iden + 'static, C: Iden + 'static>(
        &self,
        table: T,
        column: C,
    ) -> Result<(), DbErr>;
}

#[async_trait]
impl ManagerExtension for SchemaManager<'_> {
    async fn exec_sql(&self, sql: &str) -> Result<(), DbErr> {
        self.get_connection()
            .execute(Statement::from_string(
                self.get_database_backend(),
                sql.to_owned(),
            ))
            .await?;

        Ok(())
    }

    async fn sqlite_get_table_def(&self, tbl: &str) -> Result<String, DbErr> {
        self.get_connection()
            .query_one(Statement::from_string(
                DbBackend::Sqlite,
                format!("SELECT sql FROM sqlite_master WHERE name='{}';", tbl),
            ))
            .await?
            .unwrap()
            .try_get("", "sql")
    }

    async fn create_fkey<T: Iden + 'static>(
        &self,
        table: T,
        stmt: ForeignKeyCreateStatement,
    ) -> Result<(), DbErr> {
        if self.get_database_backend() == DbBackend::Sqlite {
            // Get the old table definition.
            let old_tbl_def = self.sqlite_get_table_def(&table.to_string()).await?;

            let mut new_tbl_def =
                old_tbl_def[0..old_tbl_def.rfind(")").unwrap_or(old_tbl_def.len())].to_string();

            // Write foreign key creation logic.
            let mut writer = SqlWriter::new();
            SqliteQueryBuilder.prepare_foreign_key_create_statement_internal(
                &stmt,
                &mut writer,
                Mode::Creation,
            );

            // Push prior removed ')' with new constraint.
            new_tbl_def.push_str(&format!(", {})", writer.result()));

            self.exec_sql(&sqlite_change_schema(&table.to_string(), &new_tbl_def))
                .await
        } else {
            self.create_foreign_key(stmt).await
        }
    }

    async fn drop_fkey<T: Iden + 'static, C: Iden + 'static>(
        &self,
        table: T,
        column: C,
        fkey_name: &str,
    ) -> Result<(), DbErr> {
        if self.get_database_backend() == DbBackend::Sqlite {
            // Get the old table definition.
            let tbl_def = self.sqlite_get_table_def(&table.to_string()).await?;

            let fkey_search = format!(r#"FOREIGN KEY ("{}")"#, column.to_string());
            if let Some(pos) = tbl_def.find(&fkey_search) {
                // One of these has to exist.
                let after_fkey = tbl_def[pos..tbl_def.len()].to_string();

                // Positions that are not delimiters. Such as `")`.
                let not_delim: Vec<_> = after_fkey.match_indices("\")").map(|(i, _)| i).collect();
                let mut delim: Vec<_> = after_fkey.match_indices(",").map(|e| e.0 + 1).collect();

                delim.append(
                    &mut after_fkey
                        .match_indices(")")
                        .filter(|pos| !not_delim.contains(&(pos.0 - 1)))
                        .map(|e| e.0)
                        .collect(),
                );

                self.exec_sql(&sqlite_change_schema(
                    &table.to_string(),
                    &tbl_def.replace(
                        &format!(", {}", &after_fkey[0..*delim.first().unwrap()]),
                        "",
                    ),
                ))
                .await
            } else {
                Err(DbErr::Custom(format!(
                    "Didn't find foreign key with name '{}' on table '{}'",
                    fkey_name,
                    table.to_string()
                )))
            }
        } else {
            self.drop_foreign_key(ForeignKey::drop().table(table).name(fkey_name).to_owned())
                .await
        }
    }

    async fn drop_column<T: Iden + 'static, C: Iden + 'static>(
        &self,
        table: T,
        column: C,
    ) -> Result<(), DbErr> {
        if self.get_database_backend() == DbBackend::Sqlite {
            self.exec_sql(&format!(
                "ALTER TABLE {} DROP COLUMN {};",
                table.to_string(),
                column.to_string()
            ))
            .await
        } else {
            self.alter_table(Table::alter().table(table).drop_column(column).to_owned())
                .await
        }
    }
}

/// Create query to change table schema.
fn sqlite_change_schema(tbl_name: &str, new_schema: &str) -> String {
    format!(
        r#"
        PRAGMA foreign_keys=off;

        BEGIN TRANSACTION;

        ALTER TABLE {tbl_name} RENAME TO _{tbl_name}_old;
        
        {new_schema};

        INSERT INTO {tbl_name} SELECT * FROM _{tbl_name}_old;
        DROP TABLE _{tbl_name}_old;

        COMMIT;

        PRAGMA foreign_keys=on;
        "#
    )
}
