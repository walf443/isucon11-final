use actix_multipart::MultipartError;
use actix_session::{SessionGetError, SessionInsertError};
use actix_web::HttpResponse;
use bcrypt::BcryptError;
use isucholar_core::repos::error::ReposError;
use std::io;

use isucholar_core::services;
use isucholar_core::storages::StorageError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResponseError {
    #[error("repos error")]
    ReposError(#[from] ReposError),
    #[error("storage error")]
    StorageError(#[from] StorageError),
    #[error("service error")]
    ServiceError(#[from] services::error::Error),
    #[error("actix error")]
    ActixError(#[from] actix_web::Error),
    #[error("sqlx error")]
    SqlxError(#[from] sqlx::Error),
    #[error("bcrypt error")]
    BcryptError(#[from] BcryptError),
    #[error("io error")]
    IoError(#[from] io::Error),
    #[error("multipart error")]
    MultipartError(#[from] MultipartError),
    #[error("url encode error")]
    UrlEncodeError(#[from] serde_urlencoded::ser::Error),
    #[error("session get error")]
    SessionGetError(#[from] SessionGetError),
    #[error("session insert error")]
    SessionInsertError(#[from] SessionInsertError),
    #[error("Invalid page.")]
    InvalidPage,
    #[error("Invalid file.")]
    InvalidFile,
    #[error("Code or Password is wrong.")]
    Unauthorized,
    #[error("You are already logged in.")]
    AlreadyLogin,
    #[error("An announcement with the same id already exists.")]
    AnnouncementConflict,
    #[error("No such announcement.")]
    AnnouncementNotFound,
    #[error("No such class.")]
    ClassNotFound,
    #[error("This assignment is not closed yet.")]
    ClassIsNotSubmissionClosed,
    #[error("No such course.")]
    CourseNotFound,
    #[error("This course is not in progress")]
    CourseIsNotInProgress,
    #[error("A class with the same part already exists.")]
    CourseConflict,
    #[error("No such user.")]
    UserNotFound,
    #[error("You have not taken this course.")]
    RegistrationAlready,
    #[error("Submission has been closed for this class.")]
    SubmissionClosed,
}

impl actix_web::ResponseError for ResponseError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ResponseError::Unauthorized => HttpResponse::Unauthorized()
                .content_type(mime::TEXT_PLAIN)
                .body(self.to_string()),
            ResponseError::AnnouncementNotFound
            | ResponseError::CourseNotFound
            | ResponseError::ClassNotFound => HttpResponse::NotFound()
                .content_type(mime::TEXT_PLAIN)
                .body(self.to_string()),
            ResponseError::CourseIsNotInProgress
            | ResponseError::ClassIsNotSubmissionClosed
            | ResponseError::InvalidFile
            | ResponseError::InvalidPage
            | ResponseError::RegistrationAlready
            | ResponseError::SubmissionClosed => HttpResponse::BadRequest()
                .content_type(mime::TEXT_PLAIN)
                .body(self.to_string()),
            ResponseError::CourseConflict | ResponseError::AnnouncementConflict => {
                HttpResponse::Conflict()
                    .content_type(mime::TEXT_PLAIN)
                    .body(self.to_string())
            }
            _ => {
                log::error!("{}", self);
                HttpResponse::InternalServerError()
                    .content_type(mime::TEXT_PLAIN)
                    .body(self.to_string())
            }
        }
    }
}

pub type ResponseResult<T> = std::result::Result<T, ResponseError>;

#[derive(Debug)]
pub struct SqlxError(pub sqlx::Error);
impl std::fmt::Display for SqlxError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl actix_web::ResponseError for SqlxError {
    fn error_response(&self) -> HttpResponse {
        log::error!("{}", self);
        HttpResponse::InternalServerError()
            .content_type(mime::TEXT_PLAIN)
            .body(format!("SQLx error: {:?}", self.0))
    }
}
