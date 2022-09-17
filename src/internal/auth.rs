//! Authentication logic using [`FromRequest`].
//!
//! TODO: Replace the use of traits used in generics entirely with const_generics when possible.
//! Not currently possible due to a rust compiler bug in the nightly build.
//! https://github.com/rust-lang/rust/issues/84737

use actix_web::{web::Data, Error, FromRequest, HttpRequest};
use std::ops::Deref;

use crate::{
    database::entity::users,
    models::UserRole,
    services::{auth::AuthService, ServiceError},
};

pub trait Role {
    const LEVEL: UserRole;
}

macro_rules! define_role {
    ($name:ident, $variant:expr) => {
        pub struct $name;
        impl $crate::internal::auth::Role for $name {
            const LEVEL: $crate::models::user::UserRole = $variant;
        }
    };
}

// Define all auth roles
pub mod auth_role {
    use crate::models::user::UserRole;

    define_role!(User, UserRole::User);
    define_role!(Admin, UserRole::Admin);
}

/// Define an auth option which can be used in generic parameters.
macro_rules! define_option {
    ($option:ident, $allow_name:ident, $deny_name:ident) => {
        pub trait $option {
            const ALLOW: bool;
        }

        pub struct $allow_name;
        impl $option for $allow_name {
            const ALLOW: bool = true;
        }

        pub struct $deny_name;
        impl $option for $deny_name {
            const ALLOW: bool = false;
        }
    };
}

define_option!(VerifiedOpt, AllowUnverified, DenyUnverified);
define_option!(ApplicationOpt, AllowApplication, DenyApplication);

/// Actix parameter based middleware for authentication with options.
///
/// # Arguments
///
/// * `R` - The users role. Greater roles in the underlying enum of [`auth_role`] will access to lower role access level.
/// * `VOpt` - Allow the user to be unverified. This is one of [`AllowUnverified`] or [`DenyVerified`]. This is deny by default.
/// * `AOpt` - Allow the token to be from an application. This is one of [`AllowApplication`] or [`DenyApplication`]. This is deny by default.
///
/// # Examples
///
/// ```
/// async fn route(
///     user: Auth<auth_role::User, AllowUnverified, AllowApplication>
/// ) -> Response<impl Responder> {
///     "This will permit the user to be unverified and for the token to be an application token."
/// }
/// ```
pub struct Auth<R: Role, VOpt: VerifiedOpt = DenyUnverified, AOpt: ApplicationOpt = DenyApplication>
{
    pub user: users::Model,
    _markers: (
        std::marker::PhantomData<R>,
        std::marker::PhantomData<VOpt>,
        std::marker::PhantomData<AOpt>,
    ),
}

impl<R: Role, VOpt: VerifiedOpt, AOpt: ApplicationOpt> Deref for Auth<R, VOpt, AOpt> {
    type Target = users::Model;

    fn deref(&self) -> &users::Model {
        &self.user
    }
}

impl<R: Role, VOpt: VerifiedOpt, AOpt: ApplicationOpt> FromRequest for Auth<R, VOpt, AOpt> {
    type Error = Error;
    type Future =
        std::pin::Pin<Box<dyn futures::Future<Output = Result<Auth<R, VOpt, AOpt>, Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let req = req.clone();

        Box::pin(async move {
            let auth_service = req
                .app_data::<Data<AuthService>>()
                .expect("AuthService was not found");

            let jwt_token = get_token(&req).ok_or(Error::from(ServiceError::unauthorized()))?;

            let (user, application) = match auth_service
                .validate_jwt(VOpt::ALLOW as bool, &jwt_token)
                .await
            {
                Ok(v) => v,
                Err(e) => return Err(Error::from(e)),
            };

            if (application.is_some() && !AOpt::ALLOW)
                || (UserRole::from(user.role.clone()) < R::LEVEL)
            {
                return Err(Error::from(ServiceError::unauthorized()));
            }

            Ok(Auth {
                user,
                _markers: (
                    std::marker::PhantomData,
                    std::marker::PhantomData,
                    std::marker::PhantomData,
                ),
            })
        })
    }
}

fn get_token(req: &HttpRequest) -> Option<String> {
    match req.headers().get("Authorization") {
        Some(header) => match header.to_str() {
            Ok(value) => {
                // Auth type must be bearer
                if value.starts_with("Bearer ") {
                    Some(value.trim_start_matches("Bearer ").to_string())
                } else {
                    None
                }
            }
            Err(_) => None,
        },
        None => None,
    }
}
