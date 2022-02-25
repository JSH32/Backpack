use actix_http::StatusCode;
use actix_web::{HttpResponse, Responder};
use rand::Rng;
use regex::Regex;
use sea_orm::{
    sea_query::{IntoTableRef, Query, SimpleExpr},
    ColumnTrait, ConnectionTrait, DatabaseConnection, DbErr, EntityTrait, FromQueryResult, Order,
    QueryTrait, StatementBuilder,
};
use serde_json::json;

use crate::{
    database::extensions::{QueryResultOptionExtension, QueryResultVecExtension, SelectExtension},
    models::MessageResponse,
};

pub mod auth;
pub mod file;
pub mod user;

lazy_static! {
    pub static ref EMAIL_REGEX: regex::Regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})"
    )
    .unwrap();
}

pub fn random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    return password;
}

/// Paginate a SQL table and return JSON corresponding to it
/// 
/// # Arguments:
/// 
/// * `tbl_ref` - Table to query
/// * `db` - Database connection
/// * `page_size` - Size of each page
/// * `page` - Page to query
/// * `predicate` - Optional where query for checking columns
/// * `order_by` - Optional ordering by a column
/// * `query` - Query by a column
/// * `each` - Process each result before being returned as JSON
/// 
/// TODO: Replace this with a builder pattern or extension on SeaORM
pub async fn use_paginate<R, M, C, F>(
    tbl_ref: R,
    db: &DatabaseConnection,
    page_size: u64,
    page: u64,
    predicate: Option<SimpleExpr>,
    order_by: Option<(C, Order)>,
    query: Option<(C, String)>,
    each: F,
) -> Result<impl Responder, DbErr>
where
    R: IntoTableRef + EntityTrait + Clone,
    M: FromQueryResult,
    C: ColumnTrait,
    F: FnMut(&M) -> serde_json::Value,
{
    if page < 1 {
        return Ok(
            MessageResponse::new(StatusCode::BAD_REQUEST, "Pages start at 1").http_response(),
        );
    }

    // Query to get page count
    let mut select = Query::select()
        .from(tbl_ref.to_owned().into_table_ref())
        .count()
        .to_owned();

    // Search
    if let Some((col, query)) = &query {
        select = select
            .and_where(col.like(&format!("%{}%", &query)))
            .to_owned();
    }

    // Predicate for where
    if let Some(predicate) = &predicate {
        select = select.and_where(predicate.to_owned()).to_owned();
    }

    let count: i64 = db
        .query_one(StatementBuilder::build(&select, &db.get_database_backend()))
        .await?
        .get("count")?;

    // Total amount of pages
    let total_pages = (count + page_size as i64 - 1) / page_size as i64;

    // Query to get page content
    let mut get_select = R::find()
        .query()
        .from(tbl_ref.into_table_ref())
        .limit(page_size)
        .offset((page - 1) * page_size)
        .to_owned();

    if let Some((col, query)) = &query {
        get_select = get_select
            .and_where(col.like(&format!("%{}%", &query)))
            .to_owned();
    }

    if let Some(predicate) = &predicate {
        get_select = get_select.and_where(predicate.to_owned()).to_owned();
    }

    if let Some((col, order)) = order_by {
        get_select = get_select.order_by(col, order).to_owned();
    }

    let query_response: Vec<serde_json::Value> = db
        .query_all(StatementBuilder::build(
            &get_select,
            &db.get_database_backend(),
        ))
        .await?
        .model::<M>()
        .iter()
        .map(each)
        .collect();

    if query_response.len() < 1 {
        return Ok(MessageResponse::new(
            StatusCode::NOT_FOUND,
            &format!("There are only {} pages", total_pages),
        )
        .http_response());
    }

    let mut object = serde_json::Map::new();
    object.insert("page".to_string(), json!(page));
    object.insert("pages".to_string(), json!(total_pages));
    object.insert("list".to_string(), json!(query_response));

    Ok(HttpResponse::Ok().json(serde_json::Value::Object(object)))
}
