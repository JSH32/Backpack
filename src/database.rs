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
        sqlx::query("SELECT id, email, username, password, verified, role FROM users WHERE email = $1")
            .bind(email)
            .try_map(user_map)
            .fetch_one(&self.pool)
            .await
    }
    /// Gets user info from database by id
    pub async fn get_user_by_id(&self, id: u32) -> Result<models::user::UserData, sqlx::Error> {
        sqlx::query("SELECT id, email, username, password, verified, role FROM users WHERE id = $1")
            .bind(id)
            .try_map(user_map)
            .fetch_one(&self.pool)
            .await
    }
    /// Gets user info from database by username
    pub async fn get_user_by_username(&self, username: &str) -> Result<models::user::UserData, sqlx::Error> {
        sqlx::query("SELECT id, email, username, password, verified, role FROM users WHERE username = $1")
            .bind(username)
            .try_map(user_map)
            .fetch_one(&self.pool)
            .await
    }
    /// Change a password for a user id
    pub async fn change_password(&self, id: u32, password: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET password = $1 WHERE id = $2")
            .bind(password)
            .bind(password)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
    /// Create a new token
    pub async fn create_token(&self, user_id: u32, name: &str, description: &str, token: &str) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO api_token (user_id, name, description, token) VALUES ($1, $2, $3, $4)")
            .bind(user_id)
            .bind(name)
            .bind(description)
            .bind(token)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
    /// Delete a token by its id
    pub async fn delete_token_by_id(&self, token_id: u32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM api_token WHERE id = $1")
            .bind(token_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
    /// Get a token by its id
    pub async fn get_token_by_id(&self, token_id: u32) -> Result<models::token::TokenData, sqlx::Error> {
        sqlx::query("SELECT name, description, token FROM api_token WHERE id = $1")
            .bind(token_id)
            .try_map(token_map)
            .fetch_one(&self.pool)
            .await
    }
    /// Get all tokens for a user from their id
    pub async fn get_all_tokens(&self, user_id: u32) -> Result<Vec<models::token::TokenData>, sqlx::Error> {
        sqlx::query("SELECT name, description, token FROM api_token WHERE user_id = $1")
            .bind(user_id)
            .try_map(token_map)
            .fetch_all(&self.pool)
            .await
    }
    /// Get the amount of tokens a user has
    pub async fn get_token_count(&self, user_id: u32)-> Result<i32, sqlx::Error> {
        let row: (i32,) = sqlx::query_as("SELECT COUNT(*) FROM api_token WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;
        
        Ok(row.0)
    }
    /// Check if a token already exists in the database.
    /// Return (name_exists, token_exists)
    pub async fn check_token_exist(&self, token: &str, name: &str) -> Result<(bool, bool), sqlx::Error> {
        let rows = sqlx::query("SELECT EXISTS(SELECT 1 FROM api_token WHERE name = $1) UNION ALL SELECT EXISTS(SELECT 1 FROM api_token WHERE token = $2)")
            .bind(name)
            .bind(token)
            .try_map(|row: sqlx::postgres::PgRow| -> Result<bool, sqlx::Error> {
                Ok(row.get("count"))
            })
            .fetch_all(&self.pool)
            .await?;
            
        Ok((rows[0], rows[1]))
    }
}

/// sqlx function to Map a user row to UserData
fn user_map(row: sqlx::postgres::PgRow) -> Result<models::user::UserData, sqlx::Error> {
    Ok(models::user::UserData {
        id: row.get("id"),
        email: row.get("email"),
        username: row.get("username"),
        verified: row.get("verified"),
        password: row.get("password"),
        role: row.get("role")
    })
}

/// sqlx function to Map an api token row to TokenData
fn token_map(row: sqlx::postgres::PgRow) -> Result<models::token::TokenData, sqlx::Error> {
    Ok(models::token::TokenData {
        name: row.get("name"),
        description: row.get("description"),
        token: row.get("token"),
    })
}