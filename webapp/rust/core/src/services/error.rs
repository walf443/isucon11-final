use crate::repos::error::ReposError;
use bcrypt::BcryptError;
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
    #[error("invalid password.")]
    InvalidPassword,
}

pub type Result<T> = std::result::Result<T, Error>;
