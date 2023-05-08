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

impl<'a> From<CustomError<'_>> for AppError<'a> {
    fn from(err: CustomError<'_>) -> Self {
        match err {
            CustomError::RecordDoesNotExist(record) => Self::new(
                Status::NotFound,
                Cow::from(format!("'{0}' does not exist", record)),
            ),
            CustomError::ProjectStatusAlreadyExists(status, project) => Self::new(
                Status::Conflict,
                Cow::from(format!(
                    "'{0}' already exists for project '{1}'",
                    status, project
                )),
            ),
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

impl<'a> From<DbError<'_>> for AppError<'a> {
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
pub enum DbError<'a> {
    #[error("Record already exists: {0}")]
    RecordAlreadyExists(Cow<'a, str>),
    #[error("Foreign key value does not exist")]
    ForeignKeyDoesNotExist(Cow<'a, str>),
    #[error("Internal db error")]
    InternalError,
}

impl<'a> From<diesel::result::Error> for DbError<'a> {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::DatabaseError(kind, info) => match kind {
                DatabaseErrorKind::UniqueViolation => {
                    DbError::RecordAlreadyExists(Cow::Owned(info.message().to_string()))
                }
                DatabaseErrorKind::ForeignKeyViolation => {
                    DbError::ForeignKeyDoesNotExist(Cow::Owned(
                        info.constraint_name()
                            .unwrap_or("unable to retrieve constraint")
                            .to_string(),
                    ))
                }
                _ => DbError::InternalError,
            },
            _ => DbError::InternalError,
        }
    }
}

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Token is expired")]
    ExpiredToken,
    #[error("Invalid header")]
    InvalidHeader,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Missing authorization header")]
    MissingAuthHeader,
}

impl<'a> From<AuthError> for AppError<'a> {
    fn from(err: AuthError) -> Self {
        match err {
            AuthError::ExpiredToken => Self::new(
                Status::Unauthorized,
                Cow::from(String::from("Token is expired")),
            ),
            AuthError::InvalidHeader => Self::new(
                Status::Unauthorized,
                Cow::from(String::from("Invalid header")),
            ),
            AuthError::InvalidToken => Self::new(
                Status::Unauthorized,
                Cow::from(String::from("Invalid token")),
            ),
            AuthError::MissingAuthHeader => Self::new(
                Status::Unauthorized,
                Cow::from(String::from("Missing authorization header")),
            ),
        }
    }
}
