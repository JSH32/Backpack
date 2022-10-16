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
    auth::{auth_method::AuthMethodService, new_password, validate_password},
    file::FileService,
    prelude::*,
    registration_key::RegistrationKeyService,
    ToOption,
};
use crate::{
    config::SMTPConfig,
    database::entity::{
        auth_methods, files,
        sea_orm_active_enums::{AuthMethod, Role},
        users, verifications,
    },
    internal::random_string,
};

pub struct UserService {
    database: Arc<DatabaseConnection>,
    registration_key_service: Arc<RegistrationKeyService>,
    file_service: Arc<FileService>,
    auth_method_service: Arc<AuthMethodService>,
    // If we need to send more emails, this should be split into an email service.
    smtp: Option<(AsyncSmtpTransport<Tokio1Executor>, String)>,
    client_url: String,
    use_key: bool,
}

data_service!(UserService, users);

/// TODO: Maybe try doing something like this?
/// User handle for performing operations on a user account.
/// This has permission validation built in
// pub struct UserHandle {
//     user_id: String,
//     /// User accessing the user.
//     /// If this is [`None`] then all permissions are granted.
//     accessor: Option<users::Model>,
//     database: Arc<DatabaseConnection>,

//     /// Either admin or accessor is the user themselves
//     full_permissions: bool,
// }

// impl UserHandle {

//     // pub async fn register(&self) -> ServiceResult<()> {
//     //     self.user.
//     // }
// }

impl UserService {
    pub fn new(
        database: Arc<DatabaseConnection>,
        registration_key_service: Arc<RegistrationKeyService>,
        file_service: Arc<FileService>,
        auth_method_service: Arc<AuthMethodService>,
        smtp_config: Option<SMTPConfig>,
        client_url: &str,
        use_key: bool,
    ) -> Self {
        Self {
            database,
            registration_key_service,
            file_service,
            auth_method_service,
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
    /// * `auth_method` - (method, identifier, optional username)
    /// * `registration_key` - Registration key. This can always be validated later.
    pub async fn create_user(
        &self,
        username: String,
        email: String,
        auth_method: (AuthMethod, String, Option<String>),
        registration_key: Option<String>,
    ) -> ServiceResult<users::Model> {
        validate_username(&username)?;
        if !EMAIL_REGEX.is_match(&email) {
            return Err(ServiceError::InvalidData(
                "Invalid email was provided".into(),
            ));
        }

        // Check for duplicate users.
        if let Some(v) = self
            .by_condition(
                Condition::any()
                    .add(users::Column::Username.eq(username.to_owned()))
                    .add(users::Column::Email.eq(email.to_owned())),
            )
            .await
            .to_option()?
        {
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

        // We want to validate password before making the user.
        let method_value = match auth_method.0 {
            AuthMethod::Password => new_password(&auth_method.1)?,
            _ => auth_method.1,
        };

        let registered = if self.invite_only() {
            if let Some(key) = registration_key {
                // This will validate and use the key. Will return proper error.
                self.registration_key_service.use_key(&key).await?;
                true
            } else {
                // Registration key is required for password version of this method.
                // This is not required immidiately otherwise.
                if auth_method.0 == AuthMethod::Password {
                    return Err(ServiceError::InvalidData(
                        "Registration key required".into(),
                    ));
                }
                false
            }
        } else {
            // Register user by default if invite_only is false.
            true
        };

        let user = users::ActiveModel {
            username: Set(username.to_owned()),
            email: Set(email.to_owned()),
            registered: Set(registered),
            verified: Set(!self.smtp_enabled()),
            ..Default::default()
        }
        .insert(self.database.as_ref())
        .await
        .map_err(|e| ServiceError::DbErr(e))?;

        // Create the default auth method
        auth_methods::ActiveModel {
            user_id: Set(user.id.clone()),
            auth_method: Set(auth_method.0.clone()),
            cached_username: Set(auth_method.2),
            value: Set(method_value),
            ..Default::default()
        }
        .insert(self.database.as_ref())
        .await
        .map_err(|e| ServiceError::DbErr(e))?;

        // All other methods validate email ownership.
        if AuthMethod::Password == auth_method.0 {
            // This only sends an email if SMTP is enabled.
            self.create_verification(&user).await?;
        }

        Ok(user)
    }

    /// Register an unregistered user with a registration key.
    /// Key is optional if accessing_user is [`None`] or [`Role::Admin`]
    pub async fn register_user(
        &self,
        user_id: &str,
        registration_key: Option<String>,
        accessing_user: Option<&users::Model>,
    ) -> ServiceResult<users::Model> {
        if !self.invite_only() {
            return Err(ServiceError::InvalidData(
                "Registration keys are not enabled on this service.".into(),
            ));
        }

        let user = self.by_id_authorized(user_id, accessing_user).await?;

        match registration_key {
            Some(v) => self.registration_key_service.use_key(&v).await?,
            None => {
                // Registration key can be skipped if `accessing_user` is `None` or `Role::Admin`
                if let Some(accessing_user) = accessing_user {
                    if accessing_user.role != Role::Admin {
                        return Err(ServiceError::InvalidData(
                            "Missing `registration_key`.".into(),
                        ));
                    }
                }
            }
        };

        let mut active_user = user.clone().into_active_model();
        active_user.registered = Set(true);
        active_user
            .update(self.database.as_ref())
            .await
            .map_err(|e| ServiceError::DbErr(e))
    }

    /// User specific version of `by_id_authorized`.
    pub async fn by_id_authorized(
        &self,
        user_id: &str,
        accessing_user: Option<&users::Model>,
    ) -> ServiceResult<users::Model> {
        println!("{:?}", accessing_user);
        let user_id = if user_id == "@me" {
            if let Some(accessing_user) = accessing_user {
                accessing_user.id.to_string()
            } else {
                return Err(ServiceError::InvalidData(
                    "Must be logged in to use '@me'".into(),
                ));
            }
        } else {
            user_id.into()
        };

        if let Some(accessing_user) = accessing_user {
            if accessing_user.id == user_id || accessing_user.role == Role::Admin {
                Ok(self.by_id(user_id).await?)
            } else {
                Err(ServiceError::Forbidden {
                    id: None,
                    resource: self.resource_name(),
                })
            }
        } else {
            Err(ServiceError::Forbidden {
                id: None,
                resource: self.resource_name(),
            })
        }
    }

    /// Update and validate user settings.
    ///
    /// # Arguments
    ///
    /// * `email` - New email.
    /// * `username` - New username.
    /// * `new_password` - New password.
    /// * `current_password` - Current password is required to change settings (if current password existed).
    ///
    /// # Returns
    ///
    /// The updated user model.
    pub async fn update_settings(
        &self,
        user_id: &str,
        email: Option<String>,
        username: Option<String>,
        password: Option<String>,
        current_password: Option<String>,
        accessing_user: Option<&users::Model>,
    ) -> ServiceResult<users::Model> {
        let user = self.by_id_authorized(user_id, accessing_user).await?;

        self.verify_password_action(&user, current_password, accessing_user)
            .await?;

        let mut active_user = user.clone().into_active_model();

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

        // Validate and generate password.
        // This is done last since it is a seperate operation and occurs last.
        if let Some(password) = &password {
            self.auth_method_service
                .create_or_set_method(&user.id, AuthMethod::Password, None, password)
                .await?;
        }

        if active_user.is_changed() {
            active_user
                .update(self.database.as_ref())
                .await
                .map_err(|e| ServiceError::DbErr(e))?;
        }

        // Unverify a user and send a verification email if email was changed (and SMTP enabled).
        if let Some(_) = email {
            self.create_verification(&user).await?;
        }

        Ok(self.by_id(user.id.to_owned()).await?)
    }

    /// Resend a verification code.
    /// This should be triggered only if the user is not verified.
    ///
    /// Returns [`String`] the email the verification was sent to.
    pub async fn resend_verification(
        &self,
        user_id: &str,
        accessing_user: Option<&users::Model>,
    ) -> ServiceResult<String> {
        let user = self.by_id_authorized(user_id, accessing_user).await?;

        if let None = self.smtp {
            return Err(ServiceError::Conflict("SMTP is disabled".into()));
        } else if user.verified {
            return Err(ServiceError::Conflict("User is already verified".into()));
        }

        self.create_verification(&user).await?;
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
    pub async fn delete(
        &self,
        user_id: &str,
        password: Option<String>,
        accessing_user: Option<&users::Model>,
    ) -> ServiceResult<()> {
        let user = self.by_id_authorized(user_id, accessing_user).await?;
        self.verify_password_action(&user, password, accessing_user)
            .await?;

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

    /// Verify a password required action.
    /// If password method exists on the user, validate.
    /// This returns [`ServiceError::InvalidData`] if failed.
    /// If [`None`] is provided, this always succeeds.
    ///
    /// This will always succeed if `accessing_user` is an admin trying to access a user other than themselves.
    async fn verify_password_action(
        &self,
        user: &users::Model,
        password: Option<String>,
        accessing_user: Option<&users::Model>,
    ) -> ServiceResult<()> {
        if let Some(accessing_user) = accessing_user {
            // Admins still need to verify unless accessing another user.
            if accessing_user.role == Role::Admin && user.id != accessing_user.id {
                return Ok(());
            }

            if let Some(v) = self
                .auth_method_service
                .get_auth_method(&user.id, AuthMethod::Password)
                .await
                .to_option()?
            {
                if let Some(password) = password {
                    validate_password(&v.value, &password)?
                } else {
                    return Err(ServiceError::InvalidData(
                        "Password is required since you have a password.".into(),
                    ));
                }
            }
        }

        Ok(())
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
