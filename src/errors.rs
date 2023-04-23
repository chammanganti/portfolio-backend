use std::borrow::Cow;

use diesel::result::DatabaseErrorKind;
use rocket::{
    http::Status,
    response::{status, Responder},
    serde::json::Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize, Debug)]
pub struct AppError<'a> {
    pub code: u16,
    pub status: &'a str,
    pub error: Cow<'a, str>,
}

#[derive(Error, Debug)]
pub enum CustomError<'a> {
    #[error("'{0}' does not exist")]
    RecordDoesNotExist(&'a str),
}

impl<'a> AppError<'a> {
    pub fn new(status: Status, error: Cow<'a, str>) -> Self {
        Self {
            code: status.code,
            status: status.reason().unwrap_or_default(),
            error,
        }
    }
}

impl<'a> Default for AppError<'a> {
    fn default() -> Self {
        Self {
            code: Status::InternalServerError.code,
            status: Status::InternalServerError.reason().unwrap_or_default(),
            error: Cow::from("Something went wrong"),
        }
    }
}

impl<'a> From<DbError> for AppError<'a> {
    fn from(err: DbError) -> Self {
        match err {
            DbError::RecordAlreadyExists(info) => Self::new(
                Status::Conflict,
                Cow::from(format!("Record already exists: {}", info)),
            ),
            _ => Self::default(),
        }
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for AppError<'r> {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        status::Custom(Status::new(self.code), Json(self)).respond_to(request)
    }
}

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Record already exists: {0}")]
    RecordAlreadyExists(String),
    #[error("Internal db error")]
    InternalError,
}

impl From<diesel::result::Error> for DbError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, info) => {
                DbError::RecordAlreadyExists(info.message().to_string())
            }
            _ => DbError::InternalError,
        }
    }
}