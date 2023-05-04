use crate::models::course::CourseID;
use crate::repos::error::ReposError;
use crate::storages::StorageError;
use bcrypt::BcryptError;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("test error")]
    TestError,
    #[error("repos error")]
    ReposError(#[from] ReposError),
    #[error("storage error")]
    StorageError(#[from] StorageError),
    #[error("sqlx error")]
    SqlxError(#[from] sqlx::Error),
    #[error("bcrypt error")]
    BcryptError(#[from] BcryptError),
    #[error("no such announcement.")]
    AnnouncementNotFound,
    #[error("announcement is duplicated.")]
    AnnouncementDuplicate,
    #[error("No such class.")]
    ClassNotFound,
    #[error("This assignment is not closed yet.")]
    ClassIsNotSubmissionClosed,
    #[error("no such course.")]
    CourseNotFound,
    #[error("This course is not in progress")]
    CourseIsNotInProgress,
    #[error("A class with the same part already exists.")]
    CourseConflict,
    #[error("validation error")]
    RegistrationCourseValidationError(RegistrationCourseValidationError),
    #[error("You have not taken this course.")]
    RegistrationAlready,
    #[error("Submission has been closed for this class.")]
    SubmissionClosed,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Default, Serialize)]
pub struct RegistrationCourseValidationError {
    pub course_not_found: Vec<CourseID>,
    pub not_registrable_status: Vec<CourseID>,
    pub schedule_conflict: Vec<CourseID>,
}
