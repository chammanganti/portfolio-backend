use std::borrow::Cow;

use diesel::result::DatabaseErrorKind;
use rocket::{
    http::Status,
    response::{status, Responder},
    serde::json::{json, serde_json::to_string, Json},
};
use serde::Serialize;
use thiserror::Error;
use validator::ValidationErrors;
use validator::ValidationErrorsKind::Field;

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
    #[error("'{0}' already exists for project '{1}'")]
    ProjectStatusAlreadyExists(&'a str, &'a str),
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
            DbError::ForeignKeyDoesNotExist(info) => Self::new(
                Status::BadRequest,
                Cow::from(format!(
                    "Foreign key value does not exist. Constraint: {}",
                    info
                )),
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

impl From<ValidationErrors> for AppError<'_> {
    fn from(errors: ValidationErrors) -> Self {
        let mut errs = json!({});
        for (field, errors) in errors.into_errors() {
            if let Field(errors) = errors {
                errs[field] = errors.into_iter().map(|err| err.message).collect();
            }
        }
        Self::new(
            Status::UnprocessableEntity,
            Cow::from(to_string(&errs).unwrap()),
        )
    }
}

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Record already exists: {0}")]
    RecordAlreadyExists(String),
    #[error("Foreign key value does not exist")]
    ForeignKeyDoesNotExist(String),
    #[error("Internal db error")]
    InternalError,
}

impl From<diesel::result::Error> for DbError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::DatabaseError(kind, info) => match kind {
                DatabaseErrorKind::UniqueViolation => {
                    DbError::RecordAlreadyExists(info.message().to_string())
                }
                DatabaseErrorKind::ForeignKeyViolation => DbError::ForeignKeyDoesNotExist(
                    info.constraint_name()
                        .unwrap_or("unable to retrieve constraint")
                        .to_string(),
                ),
                _ => DbError::InternalError,
            },
            _ => DbError::InternalError,
        }
    }
}
