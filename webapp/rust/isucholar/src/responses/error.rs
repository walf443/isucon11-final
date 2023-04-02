use actix_multipart::MultipartError;
use actix_web::HttpResponse;
use isucholar_core::repos::error::ReposError;
use std::io;

use thiserror::Error;
#[derive(Error, Debug)]
pub enum ResponseError {
    #[error("repos error")]
    ReposError(#[from] ReposError),
    #[error("actix error")]
    ActixError(#[from] actix_web::Error),
    #[error("sqlx error")]
    SqlxError(#[from] sqlx::Error),
    #[error("io error")]
    IoError(#[from] io::Error),
    #[error("multipart error")]
    MultipartError(#[from] MultipartError),
    #[error("url encode error")]
    UrlEncodeError(#[from] serde_urlencoded::ser::Error),
    #[error("Invalid page.")]
    InvalidPage,
    #[error("Invalid file.")]
    InvalidFile,
    #[error("An announcement with the same id already exists.")]
    AnnouncementConflict,
    #[error("No such class.")]
    ClassNotFound,
    #[error("No such course.")]
    CourseNotFound,
    #[error("This assignment is not closed yet.")]
    ClassIsNotSubmissionClosed,
    #[error("This course is not in progress")]
    CourseIsNotInProgress,
    #[error("A class with the same part already exists.")]
    CourseConflict,
    #[error("You have not taken this course.")]
    RegistrationAlready,
    #[error("Submission has been closed for this class.")]
    SubmissionClosed,
}

impl actix_web::ResponseError for ResponseError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ResponseError::CourseNotFound | ResponseError::ClassNotFound => {
                HttpResponse::NotFound()
                    .content_type(mime::TEXT_PLAIN)
                    .body(self.to_string())
            }
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
