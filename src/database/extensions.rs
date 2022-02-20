use sea_orm::{
    sea_query::{Expr, SelectStatement},
    ColumnTrait, DbBackend, DbErr, FromQueryResult, QueryResult, Statement, StatementBuilder,
    TryGetable,
};

pub trait SelectExtension {
    fn build_statement(&self, backend: &DbBackend) -> Statement;
    fn count(&mut self) -> &mut Self;
    fn search<C: ColumnTrait>(&mut self, column: C, query: &str) -> &mut Self;
    fn page(&mut self, page_size: u64, page: u64) -> &mut Self;
}

impl SelectExtension for SelectStatement {
    fn build_statement(&self, backend: &DbBackend) -> Statement {
        StatementBuilder::build(self, backend)
    }

    fn count(&mut self) -> &mut Self {
        self.expr(Expr::cust("COUNT(*)"))
            .expr(Expr::expr(Expr::cust("*")).count())
    }

    fn search<C: ColumnTrait>(&mut self, column: C, query: &str) -> &mut Self {
        self.and_where(column.like(&format!("%{}%", &query)))
    }

    fn page(&mut self, page_size: u64, page: u64) -> &mut Self {
        self.limit(page_size).offset((page - 1) * page_size)
    }
}

pub trait QueryResultOptionExtension {
    fn get<T: TryGetable>(&mut self, column: &str) -> Result<T, DbErr>;
}

impl QueryResultOptionExtension for Option<QueryResult> {
    fn get<T: TryGetable>(&mut self, column: &str) -> Result<T, DbErr> {
        Ok(T::try_get(
            &self
                .take()
                .ok_or(DbErr::Custom(format!("Failed to get column: '{}'", column)))?,
            "",
            column,
        )?)
    }
}

pub trait QueryResultVecExtension {
    fn model<T: FromQueryResult>(&self) -> Vec<T>;
}

impl QueryResultVecExtension for Vec<QueryResult> {
    fn model<T: FromQueryResult>(&self) -> Vec<T> {
        self.iter()
            .map(|v| T::from_query_result(v, ""))
            .flatten()
            .collect()
    }
}
