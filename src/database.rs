use crate::{models};

use sqlx::postgres::PgPoolOptions;
use sqlx::Row;

pub struct Database {
    pool: sqlx::Pool<sqlx::Postgres>
}

impl Database {
    pub async fn new(max_connections: u32, url: &str) -> Self {
        Database {
            pool: PgPoolOptions::new()
                        .max_connections(max_connections)
                        .connect(url).await
                        .expect("Could not initialize connection")
        }
    }
    /// Creates a user from a user creation form
    pub async fn create_user(&self, form: &models::user::UserCreateForm) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO users (email, username, password) VALUES ($1, $2, $3)")
            .bind(&form.email)
            .bind(&form.username)
            .bind(&form.password)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
    /// Gets user info from database by email
    pub async fn get_user_by_email(&self, email: &str) -> Result<models::user::UserData, sqlx::Error> {
        sqlx::query("SELECT id, email, username, password, verified FROM users WHERE email = $1")
            .bind(email)
            .try_map(|row: sqlx::postgres::PgRow| {
                Ok(models::user::UserData {
                    id: row.get("id"),
                    email: row.get("email"),
                    username: row.get("username"),
                    verified: row.get("verified"),
                    password: row.get("password")
                })
            })
            .fetch_one(&self.pool)
            .await
    }
    /// Gets user info from database by id
    pub async fn get_user_by_id(&self, id: u32) -> Result<models::user::UserData, sqlx::Error> {
        sqlx::query("SELECT id, email, username, password, verified FROM users WHERE id = $1")
            .bind(id)
            .try_map(|row: sqlx::postgres::PgRow| {
                Ok(models::user::UserData {
                    id: row.get("id"),
                    email: row.get("email"),
                    username: row.get("username"),
                    verified: row.get("verified"),
                    password: row.get("password")
                })
            })
            .fetch_one(&self.pool)
            .await
    }
    /// Gets user info from database by username
    pub async fn get_user_by_username(&self, username: &str) -> Result<models::user::UserData, sqlx::Error> {
        sqlx::query("SELECT id, email, username, password, verified FROM users WHERE username = $1")
            .bind(username)
            .try_map(|row: sqlx::postgres::PgRow| {
                Ok(models::user::UserData {
                    id: row.get("id"),
                    email: row.get("email"),
                    username: row.get("username"),
                    verified: row.get("verified"),
                    password: row.get("password")
                })
            })
            .fetch_one(&self.pool)
            .await
    }
}