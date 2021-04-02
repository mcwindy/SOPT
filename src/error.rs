use crate::data::GeneralResponse;
use actix_web::{HttpResponse, ResponseError};
use sqlx::Error as DBError;
use std::fmt::{Display, Formatter};

/// General error types. it contains:
/// 1. AuthError, triggered when jwt not set or expired
/// 2. DBError, wrapper for sqlx `Error`
/// 3. OtherError, with a generic to rewrite other errors to string
///     like utilities and standard library error.
/// 4. NotFound
/// 5. No permission in this account
/// 6. Request Error
///
/// All errors will be transformed to Http Response so no panic will happen.
#[derive(Debug)]
pub enum Error {
    AuthError,
    OtherError(String),
    DBError(DBError),
    RocksDBError(rocksdb::Error),
    NotFound,
    NoPermission,
    RequestError(String),
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Error::DBError(err)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::OtherError(err)
    }
}

impl From<rocksdb::Error> for Error {
    fn from(err: rocksdb::Error) -> Self {
        Error::RocksDBError(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for Error {}

impl ResponseError for Error {
    /// Transform error messages to Http Response.
    fn error_response(&self) -> HttpResponse {
        match *self {
            Error::NotFound => {
                HttpResponse::Ok().json(GeneralResponse::from_err("DB record not found"))
            }
            Error::DBError(ref err) => HttpResponse::InternalServerError().body(err.to_string()),
            Error::RocksDBError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
            Error::AuthError => {
                HttpResponse::Unauthorized().json(GeneralResponse::from_err("Not login yet"))
            }
            Error::NoPermission => {
                HttpResponse::Unauthorized().json(GeneralResponse::from_err("No permission"))
            }
            Error::OtherError(ref err) => {
                HttpResponse::InternalServerError().json(GeneralResponse::from_err(err))
            }
            Error::RequestError(ref err) => {
                HttpResponse::UnprocessableEntity().json(GeneralResponse::from_err(err))
            }
        }
    }
}

/// A generic function to be applied to any
/// type which implements Error trait.
/// They will be turned into strings of OtherError.
///
/// Especially useful when comes to `map_err()` case.
pub fn error_string<T>(err: T) -> Error
where
    T: std::error::Error,
{
    Error::OtherError(err.to_string())
}
