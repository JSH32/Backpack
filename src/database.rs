use std::path::Path;
use std::error::Error as StdError;

use crate::models::UserData;
use crate::models::{self, application::ApplicationData};
use crate::sonyflake::{self, Sonyflake};

use sqlx::migrate::Migrator;
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("database error")]
    SqlxError(sqlx::Error),
    #[error("there was a problem generating a sonyflake")]
    SonyflakeError(sonyflake::Error)
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Error {
        Error::SqlxError(error)
    }
}

impl From<sonyflake::Error> for Error {
    fn from(error: sonyflake::Error) -> Error {
        Error::SonyflakeError(error)
    }
}

pub struct Database {
    pool: sqlx::Pool<sqlx::Postgres>,
    sonyflake: Sonyflake
}

impl Database {
    pub async fn new(max_connections: u32, url: &str, sonyflake_worker: Sonyflake) -> Self {
        Database {
            pool: PgPoolOptions::new()
                        .max_connections(max_connections)
                        .connect(url).await
                        .expect("Could not initialize connection"),
            sonyflake: sonyflake_worker
        }
    }

    fn get_sonyflake(&self) -> Result<String, Error> {
        match self.sonyflake.next_id() {
            Ok(val) => Ok(val.to_string()),
            Err(err) => Err(Error::SonyflakeError(err))
        }
    }

    /// Run all pending up migrations
    pub async fn run_migrations(&self, path: &Path) -> Result<(), Box<dyn StdError>> {
        let migrator = Migrator::new(path).await?;
        migrator.run(&self.pool).await?;

        Ok(())
    }

    /// Creates a user from a user creation form
    pub async fn create_user(&self, form: &models::user::UserCreateForm) -> Result<UserData, Error> {
        sqlx::query("INSERT INTO users (id, email, username, password) VALUES ($1, $2, $3, $4) RETURNING *")
            .bind(&self.get_sonyflake()?)
            .bind(&form.email)
            .bind(&form.username)
            .bind(&form.password)
            .try_map(user_map)
            .fetch_one(&self.pool)
            .await
            .map_err(From::from)
    }
    /// Gets user info from database by email
    pub async fn get_user_by_email(&self, email: &str) -> Result<UserData, Error> {
        sqlx::query("SELECT id, email, username, password, verified, role FROM users WHERE email = $1")
            .bind(email)
            .try_map(user_map)
            .fetch_one(&self.pool)
            .await
            .map_err(From::from)
    }
    /// Gets user info from database by id
    pub async fn get_user_by_id(&self, id: &str) -> Result<UserData, Error> {
        sqlx::query("SELECT id, email, username, password, verified, role FROM users WHERE id = $1")
            .bind(id)
            .try_map(user_map)
            .fetch_one(&self.pool)
            .await
            .map_err(From::from)
    }

    /// Gets user info from database by username
    pub async fn get_user_by_username(&self, username: &str) -> Result<models::user::UserData, Error> {
        sqlx::query("SELECT id, email, username, password, verified, role FROM users WHERE username = $1")
            .bind(username)
            .try_map(user_map)
            .fetch_one(&self.pool)
            .await
            .map_err(From::from)
    }

    /// Delete a user and all their tokens
    pub async fn delete_user(&self, id: &str) -> Result<(), Error> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Change a password for a user id
    pub async fn change_password(&self, id: &str, password: &str) -> Result<(), Error> {
        sqlx::query("UPDATE users SET password = $1 WHERE id = $2")
            .bind(password)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Delete an application by its id
    pub async fn delete_application_by_id(&self, application_id: &str) -> Result<(), Error> {
        sqlx::query("DELETE FROM applications WHERE id = $1")
            .bind(application_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Get an application by its id
    pub async fn get_application_by_id(&self, application_id: &str) -> Result<ApplicationData, Error> {
        sqlx::query("SELECT id, name, user_id FROM applications WHERE id = $1")
            .bind(application_id)
            .try_map(application_map)
            .fetch_one(&self.pool)
            .await
            .map_err(From::from)
    }
    
    /// Get all applications for a user from their id
    pub async fn get_all_applications(&self, user_id: &str) -> Result<Vec<ApplicationData>, Error> {
        sqlx::query("SELECT id, name, user_id FROM applications WHERE user_id = $1")
            .bind(user_id)
            .try_map(application_map)
            .fetch_all(&self.pool)
            .await
            .map_err(From::from)
    }

    /// Create a new application
    pub async fn create_application(&self, user_id: &str, name: &str) -> Result<ApplicationData, Error> {
        sqlx::query("INSERT INTO applications (id, user_id, name) VALUES ($1, $2, $3) RETURNING id, user_id, name")
            .bind(&self.get_sonyflake()?)
            .bind(user_id)
            .bind(name)
            .try_map(application_map)
            .fetch_one(&self.pool)
            .await
            .map_err(From::from)
    }

    /// Check if a token of that name already exists for a user
    pub async fn application_exist(&self, user_id: &str, name: &str) -> Result<bool, Error> {
        let row: (bool,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM applications WHERE name = $1 AND user_id = $2)")
            .bind(name)
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(row.0)
    }

    /// Get the amount of applications a user has
    pub async fn application_count(&self, user_id: &str)-> Result<i64, Error> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM applications WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;
        
        Ok(row.0)
    }

    pub async fn create_verification(&self, user_id: &str, code: &str) -> Result<(), Error> {
        sqlx::query("INSERT INTO verifications (user_id, code) VALUES ($1, $2)")
            .bind(user_id)
            .bind(code)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // SELECT id, email, username, password, verified, role FROM users WHERE id = $1
    pub async fn get_user_from_verification(&self, code: &str) -> Result<UserData, Error> {
        sqlx::query("SELECT u.id, u.email, u.username, u.password, u.verified, u.role FROM users u JOIN verifications v ON v.user_id = u.id WHERE v.code = $1")
            .bind(code)
            .try_map(user_map)
            .fetch_one(&self.pool)
            .await
            .map_err(From::from)
    }
    
    pub async fn delete_verification(&self, user_id: &str) -> Result<(), Error> {
        sqlx::query("DELETE FROM verifications WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }

    /// Verify the user
    pub async fn verify_user(&self, user_id: &str) -> Result<(), Error> {
        sqlx::query("UPDATE users SET verified = TRUE WHERE id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
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

fn application_map(row: sqlx::postgres::PgRow) -> Result<ApplicationData, sqlx::Error> {
    Ok(ApplicationData {
        id: row.get("id"),
        name: row.get("name"),
        user_id:  row.get("user_id"),
        token: None
    })
}