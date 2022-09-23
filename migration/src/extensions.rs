use sea_orm_migration::{
    async_trait::async_trait,
    sea_orm::{ConnectionTrait, Statement},
    sea_query::ColumnDef,
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
}
