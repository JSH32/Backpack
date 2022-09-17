use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    Tokio1Executor,
};
use regex::Regex;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel,
    ModelTrait, QueryFilter, Set,
};
use std::sync::Arc;

use super::{
    auth::{new_password, validate_password},
    file::FileService,
    prelude::*,
    registration_key::RegistrationKeyService,
};
use crate::{
    config::SMTPConfig,
    database::entity::{files, users, verifications},
    internal::random_string,
};

pub struct UserService {
    database: Arc<DatabaseConnection>,
    registration_key_service: Arc<RegistrationKeyService>,
    file_service: Arc<FileService>,
    // If we need to send more emails, this should be split into an email service.
    smtp: Option<(AsyncSmtpTransport<Tokio1Executor>, String)>,
    client_url: String,
    use_key: bool,
}

data_service!(UserService, users);

impl UserService {
    pub fn new(
        database: Arc<DatabaseConnection>,
        registration_key_service: Arc<RegistrationKeyService>,
        file_service: Arc<FileService>,
        smtp_config: Option<SMTPConfig>,
        client_url: &str,
        use_key: bool,
    ) -> Self {
        Self {
            database,
            registration_key_service,
            file_service,
            smtp: match smtp_config {
                Some(config) => {
                    let creds = Credentials::new(config.username.clone(), config.password);

                    Some((
                        AsyncSmtpTransport::<Tokio1Executor>::relay(&config.server)
                            .unwrap()
                            .credentials(creds)
                            .build(),
                        config.username,
                    ))
                }
                None => None,
            },
            client_url: client_url.to_owned(),
            use_key,
        }
    }

    /// Is SMTP enabled?
    pub fn smtp_enabled(&self) -> bool {
        self.smtp.is_some()
    }

    /// Are registration keys required?
    pub fn invite_only(&self) -> bool {
        self.use_key
    }

    /// Get a user by their username or email.
    pub async fn get_by_identifier(&self, identifier: &str) -> ServiceResult<users::Model> {
        self.by_condition(
            Condition::any().add(
                if EMAIL_REGEX.is_match(&identifier) {
                    users::Column::Email
                } else {
                    users::Column::Username
                }
                .eq(identifier.to_string()),
            ),
        )
        .await
    }

    /// Create a user.
    ///
    /// # Arguments
    ///
    /// * `username` - Users username
    /// * `email` - User email
    /// * `password` - User password (this will be hashed)
    /// * `key` - Registration key.
    pub async fn create_user(
        &self,
        username: String,
        email: String,
        password: String,
        key: Option<String>,
    ) -> ServiceResult<users::Model> {
        validate_username(&username)?;
        if !EMAIL_REGEX.is_match(&email) {
            return Err(ServiceError::InvalidData(
                "Invalid email was provided".into(),
            ));
        }

        // Check for duplicate users.
        match self
            .by_condition(
                Condition::any()
                    .add(users::Column::Username.eq(username.to_owned()))
                    .add(users::Column::Email.eq(email.to_owned())),
            )
            .await
        {
            Ok(v) => {
                // User was found, report proper error based on field.
                return Err(ServiceError::Conflict(format!(
                    "An account with that {} already exists!",
                    if username == v.username {
                        "username"
                    // The only other intended fail reason would be email.
                    } else {
                        "email"
                    }
                )));
            }
            Err(e) => match e {
                // Only error if there was an actual error.
                // Not finding a result is intended.
                ServiceError::NotFound(_) => {}
                _ => return Err(e),
            },
        }

        // Use the registration key.
        if self.use_key {
            if let Some(key) = key {
                // This will validate and use the key. Will return proper error.
                self.registration_key_service.use_key(&key).await?;
            } else {
                return Err(ServiceError::InvalidData(
                    "Registration key required".into(),
                ));
            }
        }

        let user = users::ActiveModel {
            username: Set(username.to_owned()),
            email: Set(email.to_owned()),
            password: Set(new_password(&password)?),
            ..Default::default()
        }
        .insert(self.database.as_ref())
        .await
        .map_err(|e| ServiceError::DbErr(e))?;

        // This only sends an email if SMTP is enabled.
        self.create_verification(&user).await?;

        Ok(user)
    }

    /// Update and validate user settings.
    ///
    /// # Arguments
    ///
    /// * `email` - New email.
    /// * `username` - New username.
    /// * `new_password` - New password.
    /// * `current_password` - Current password is required to change settings.
    ///
    /// # Returns
    ///
    /// The updated user model.
    pub async fn update_settings(
        &self,
        user: &users::Model,
        email: Option<String>,
        username: Option<String>,
        password: Option<String>,
        current_password: String,
    ) -> ServiceResult<users::Model> {
        validate_password(&user.password, &current_password)?;

        let mut active_user = user.clone().into_active_model();

        // Validate and generate password.
        if let Some(password) = &password {
            active_user.password = Set(new_password(&password)?);
        }

        // Validate username.
        if let Some(username) = &username {
            validate_username(&username)?;

            if users::Entity::find()
                .filter(users::Column::Username.eq(username.to_owned()))
                .one(self.database.as_ref())
                .await
                .map_err(|e| ServiceError::DbErr(e))?
                .is_some()
            {
                return Err(ServiceError::Conflict(
                    "An account with that username already exists!".into(),
                ));
            }

            active_user.username = Set(username.to_owned());
        }

        // Validate email.
        if let Some(email) = &email {
            if !EMAIL_REGEX.is_match(&email) {
                return Err(ServiceError::InvalidData(
                    "Invalid email was provided".into(),
                ));
            }

            if users::Entity::find()
                .filter(users::Column::Email.eq(email.to_owned()))
                .one(self.database.as_ref())
                .await
                .map_err(|e| ServiceError::DbErr(e))?
                .is_some()
            {
                return Err(ServiceError::Conflict(
                    "An account with that email already exists!".into(),
                ));
            }

            active_user.email = Set(email.to_owned());
        }

        active_user
            .update(self.database.as_ref())
            .await
            .map_err(|e| ServiceError::DbErr(e))?;

        // Unverify a user and send a verification email if email was changed (and SMTP enabled).
        if let Some(_) = email {
            self.create_verification(user).await?;
        }

        Ok(self.by_id(user.id.to_owned()).await?)
    }

    /// Resend a verification code.
    /// This should be triggered only if the user is not verified.
    ///
    /// Returns [`String`] the email the verification was sent to.
    pub async fn resend_verification(&self, user: &users::Model) -> ServiceResult<String> {
        if let None = self.smtp {
            return Err(ServiceError::Conflict("SMTP is disabled".into()));
        } else if user.verified {
            return Err(ServiceError::Conflict("User is already verified".into()));
        }

        self.create_verification(user).await?;
        Ok(user.email.to_owned())
    }

    /// Verify a user based on the user themselves.
    pub async fn verify_user(&self, user: &users::Model) -> ServiceResult<()> {
        verifications::Entity::delete_many()
            .filter(verifications::Column::UserId.eq(user.id.to_owned()))
            .exec(self.database.as_ref())
            .await
            .map_err(|e| ServiceError::DbErr(e))?;

        users::ActiveModel {
            id: Set(user.id.to_owned()),
            verified: Set(true),
            ..Default::default()
        }
        .update(self.database.as_ref())
        .await
        .map_err(|e| ServiceError::DbErr(e))?;

        Ok(())
    }

    /// Verify a user by a verification code.
    pub async fn verify_by_code(&self, code: &str) -> ServiceResult<()> {
        if let None = self.smtp {
            return Err(ServiceError::Conflict("SMTP is disabled".into()));
        }

        match verifications::Entity::find()
            .filter(verifications::Column::Code.eq(code.to_owned()))
            .find_also_related(users::Entity)
            .one(self.database.as_ref())
            .await
            .map_err(|e| ServiceError::DbErr(e))?
        {
            Some((verification, user_data_opt)) => {
                // This can't really be None
                let user_data = user_data_opt.unwrap();

                // Delete the verification.
                verification
                    .delete(self.database.as_ref())
                    .await
                    .map_err(|e| ServiceError::DbErr(e))?;

                // Verify the user
                let mut active_user: users::ActiveModel = user_data.into();
                active_user.verified = Set(true);
                active_user
                    .update(self.database.as_ref())
                    .await
                    .map_err(|e| ServiceError::DbErr(e))?;

                Ok(())
            }
            None => Err(ServiceError::InvalidData(
                "Invalid verification code was provided".into(),
            )),
        }
    }

    /// Delete a user.
    ///
    /// # Arguments
    ///
    /// * `user` - User to delete.
    /// * `password` - If provided, will make sure that the correct password is provided.
    pub async fn delete(&self, user: &users::Model, password: Option<&str>) -> ServiceResult<()> {
        if let Some(password) = password {
            validate_password(&user.password, password)?;
        }

        let mut files: Vec<String> = user
            .find_related(files::Entity)
            .all(self.database.as_ref())
            .await
            .map_err(|e| ServiceError::DbErr(e))?
            .iter()
            .map(|f| (f.name.to_owned()))
            .collect();

        // Add all thumbnails, not all files will have thumbnails but this will work.
        files.extend(
            files
                .iter()
                .map(|f| format!("thumb/{}", f))
                .collect::<Vec<String>>(),
        );

        // Delete the user before deleting the files.
        // File deletion may take a while, if something happens to the server we would rather keep the actual files rather than the records.
        // This is also to prevent the user from doing anything while the operation is occuring.
        user.clone()
            .delete(self.database.as_ref())
            .await
            .map_err(|e| ServiceError::DbErr(e))?;

        // Delete every file.
        let _ = self.file_service.storage.delete_objects(files).await;

        Ok(())
    }

    /// Create and send verification email.
    /// This will unverify a user if they are currently verified and delete currently existing verifications.
    ///
    /// This will only work if smtp is enabled.
    ///
    /// Returns [`bool`] whether the verification was created or not.
    async fn create_verification(&self, user: &users::Model) -> ServiceResult<bool> {
        // Send the user an email
        Ok(if let Some(smtp) = &self.smtp {
            let random_code = random_string(72);

            // Delete all old verifications which may exist.
            verifications::Entity::delete_many()
                .filter(verifications::Column::UserId.eq(user.id.to_owned()))
                .exec(self.database.as_ref())
                .await
                .map_err(|e| ServiceError::DbErr(e))?;

            // Create a new verification
            verifications::ActiveModel {
                user_id: Set(user.id.to_owned()),
                code: Set(random_code.to_owned()),
                ..Default::default()
            }
            .insert(self.database.as_ref())
            .await
            .map_err(|e| ServiceError::DbErr(e))?;

            // Send the email.
            let email = verification_email(&self.client_url, &smtp.1, &user.email, &random_code);
            let mailer = smtp.clone().0;
            tokio::spawn(async move {
                let _ = mailer.send(email).await;
            });

            // Unverify the user if they are verified.
            if user.verified {
                let mut user = user.clone().into_active_model();
                user.verified = Set(false);
                user.update(self.database.as_ref())
                    .await
                    .map_err(|e| ServiceError::DbErr(e))?;
            }

            true
        } else {
            false
        })
    }
}

lazy_static! {
    static ref USERNAME_REGEX: regex::Regex = Regex::new(r"^[A-Za-z][A-Za-z0-9_]{4,14}$").unwrap();
    pub static ref EMAIL_REGEX: regex::Regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})"
    )
    .unwrap();
}

/// Create verification email.
fn verification_email(client_url: &str, from_email: &str, email: &str, code: &str) -> Message {
    Message::builder()
        .from(from_email.parse().unwrap())
        .to(email.parse().unwrap())
        .subject("Verify your account")
        .body(
            format!(
                "Please click on this link to verify your account\n{}user/verify?code={}",
                client_url, code
            )
            .to_string(),
        )
        .unwrap()
}

/// Validate a username.
fn validate_username(username: &str) -> ServiceResult<()> {
    let username_length = username.len();
    if username_length < 5 {
        Err(ServiceError::InvalidData(
            "Username too short (minimum 5 characters)".into(),
        ))
    } else if username_length > 15 {
        Err(ServiceError::InvalidData(
            "Username too long (maximum 15 characters)".into(),
        ))
    } else if !USERNAME_REGEX.is_match(username) {
        Err(ServiceError::InvalidData(
            "Username may not contain any symbols".into(),
        ))
    } else {
        Ok(())
    }
}
