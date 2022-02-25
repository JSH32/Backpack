use sea_orm::{
    sea_query::{Expr, SelectStatement},
    DbErr, FromQueryResult, QueryResult,
    TryGetable,
};


pub trait SelectExtension {
    fn count(&mut self) -> &mut Self;
}

impl SelectExtension for SelectStatement {
    fn count(&mut self) -> &mut Self {
        self.expr(Expr::cust("COUNT(*)"))
            .expr(Expr::expr(Expr::cust("*")).count())
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
