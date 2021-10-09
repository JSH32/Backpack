use thiserror::Error;

use super::sonyflake;

#[derive(Error, Debug)]
pub enum Error {
    #[error("database error: `{0}`")]
    SqlxError(sqlx::Error),
    #[error("there was a problem generating a sonyflake: `{0}`")]
    SonyflakeError(sonyflake::Error),
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
