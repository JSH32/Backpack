pub mod sonyflake;
pub mod error;

use std::path::Path;

use crate::models::UserData;
use crate::models::file::FileData;
use crate::models::{self, application::ApplicationData};

use chrono::{DateTime, Utc};
use sqlx::migrate::{MigrateError, Migrator};
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;

use self::error::Error;
use self::sonyflake::Sonyflake;

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
    pub async fn run_migrations(&self, path: &Path) -> Result<(), MigrateError> {
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
    pub async fn change_password(&self, user_id: &str, password: &str) -> Result<(), Error> {
        sqlx::query("UPDATE users SET password = $1 WHERE id = $2")
            .bind(password)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn change_email(&self, user_id: &str, email: &str) -> Result<(), Error> {
        sqlx::query("UPDATE users SET email = $1 WHERE id = $2")
            .bind(email)
            .bind(user_id)
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
        sqlx::query("INSERT INTO verifications (user_id, code) VALUES ($1, $2) ON CONFLICT (user_id) DO UPDATE SET code = $2")
            .bind(user_id)
            .bind(code)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

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
    pub async fn verify_user(&self, user_id: &str, verify: bool) -> Result<(), Error> {
        sqlx::query("UPDATE users SET verified = $1 WHERE id = $2")
            .bind(verify)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn get_file(&self, id: &str) -> Result<FileData, Error> {
        sqlx::query("SELECT id, name, uploader, hash, uploaded, size FROM files WHERE id = $1")
            .bind(id)
            .try_map(file_map)
            .fetch_one(&self.pool)
            .await
            .map_err(From::from)
    }

    pub async fn exist_file_hash(&self, user_id: &str, hash: &str) -> Result<Option<String>, Error> {
        match sqlx::query("SELECT name FROM files WHERE uploader = $1 AND hash = $2")
            .bind(user_id)
            .bind(hash)
            .fetch_one(&self.pool)
            .await {
                Ok(row) => Ok(Some(row.get("name"))),
                Err(err) => match err {
                    sqlx::Error::RowNotFound => Ok(None),
                    _ => Err(SqlxError(err))
                },
            }
    }

    pub async fn create_file(&self, user_id: &str, name: &str, hash: &str, size: i32, uploaded: DateTime<Utc>) -> Result<FileData, Error> {
        sqlx::query("INSERT INTO files (id, uploader, name, hash, size, uploaded) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *")
            .bind(self.get_sonyflake()?)
            .bind(user_id)
            .bind(name)
            .bind(hash)
            .bind(size)
            .bind(uploaded)
            .try_map(file_map)
            .fetch_one(&self.pool)
            .await
            .map_err(From::from)
    }

    pub async fn delete_file(&self, id: &str) -> Result<(), Error> {
        sqlx::query("DELETE FROM files WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }

    pub async fn get_files(&self, user_id: &str, limit: u32, page: u32) -> Result<Vec<FileData>, Error> {
        sqlx::query("SELECT * FROM files WHERE uploader = $1 ORDER BY uploaded DESC LIMIT $2 OFFSET $3")
            .bind(user_id)
            .bind(limit)
            .bind((page - 1) * limit)
            .try_map(file_map)
            .fetch_all(&self.pool)
            .await
            .map_err(From::from)
    }

    pub async fn get_total_file_pages(&self, user_id: &str, page_size: u32) -> Result<u32, Error> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM files WHERE uploader = $1")
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;
        
        Ok((row.0 as u32 + page_size - 1) / page_size)
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

fn file_map(row: sqlx::postgres::PgRow) -> Result<models::file::FileData, sqlx::Error> {
    Ok(models::file::FileData {
        id: row.get("id"),
        uploader: row.get("uploader"),
        name: row.get("name"),
        hash: row.get("hash"),
        uploaded: row.get("uploaded"),
        size: row.get("size"),
        url: None
    })
}

fn application_map(row: sqlx::postgres::PgRow) -> Result<models::application::ApplicationData, sqlx::Error> {
    Ok(models::application::ApplicationData {
        id: row.get("id"),
        name: row.get("name"),
        user_id:  row.get("user_id"),
        token: None
    })
}