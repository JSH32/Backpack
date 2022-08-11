use sea_orm_migration::sea_query::ColumnDef;

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
