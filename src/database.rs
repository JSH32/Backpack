use crate::models;

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
    pub async fn get_user_by_id(&self, id: i32) -> Result<models::user::UserData, sqlx::Error> {
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
    /// Delete a user and all their tokens
    pub async fn delete_user(&self, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
    /// Change a password for a user id
    pub async fn change_password(&self, id: i32, password: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET password = $1 WHERE id = $2")
            .bind(password)
            .bind(password)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
    /// Delete a token by its id
    pub async fn delete_token_by_id(&self, token_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM tokens WHERE id = $1")
            .bind(token_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
    /// Get a token by its id
    pub async fn get_token_by_id(&self, token_id: i32) -> Result<models::token::TokenData, sqlx::Error> {
        sqlx::query("SELECT id, name, user_id FROM tokens WHERE id = $1")
            .bind(token_id)
            .try_map(token_map)
            .fetch_one(&self.pool)
            .await
    }
    /// Get all tokens for a user from their id
    pub async fn get_all_tokens(&self, user_id: i32) -> Result<Vec<models::token::TokenData>, sqlx::Error> {
        sqlx::query("SELECT id, name, user_id FROM tokens WHERE user_id = $1")
            .bind(user_id)
            .try_map(token_map)
            .fetch_all(&self.pool)
            .await
    }
    /// Create a new token
    pub async fn create_token(&self, user_id: i32, name: &str) -> Result<models::token::TokenData, sqlx::Error> {
        sqlx::query("INSERT INTO tokens (user_id, name) VALUES ($1, $2) RETURNING id, user_id, name")
            .bind(user_id)
            .bind(name)
            .try_map(token_map)
            .fetch_one(&self.pool)
            .await
    }
    /// Check if a token of that name already exists in the database
    pub async fn token_exist(&self, user_id: i32, name: &str) -> Result<bool, sqlx::Error> {
        let row: (bool,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM tokens WHERE name = $1 AND user_id = $2)")
            .bind(name)
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(row.0)
    }
    /// Get the amount of tokens a user has
    pub async fn token_count(&self, user_id: i32)-> Result<i64, sqlx::Error> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tokens WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;
        
        Ok(row.0)
    }
}

/// sqlx function to map a user row to UserData
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

fn token_map(row: sqlx::postgres::PgRow) -> Result<models::token::TokenData, sqlx::Error> {
    Ok(models::token::TokenData {
        id: row.get("id"),
        name: row.get("name"),
        user_id: row.get("user_id"),
        token: None
    })
}