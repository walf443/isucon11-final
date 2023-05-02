use crate::models::course::CourseID;
use crate::repos::error::ReposError;
use bcrypt::BcryptError;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("test error")]
    TestError,
    #[error("repos error")]
    ReposError(#[from] ReposError),
    #[error("sqlx error")]
    SqlxError(#[from] sqlx::Error),
    #[error("bcrypt error")]
    BcryptError(#[from] BcryptError),
    #[error("no such announcement.")]
    AnnouncementNotFound,
    #[error("announcement is duplicated.")]
    AnnouncementDuplicate,
    #[error("no such course.")]
    CourseNotFound,
    #[error("validation error")]
    RegistrationCourseValidationError(RegistrationCourseValidationError),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Default, Serialize)]
pub struct RegistrationCourseValidationError {
    pub course_not_found: Vec<CourseID>,
    pub not_registrable_status: Vec<String>,
    pub schedule_conflict: Vec<String>,
}
