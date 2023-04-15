use crate::repos::error::ReposError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("test error")]
    TestError,
    #[error("repos error")]
    ReposError(#[from] ReposError),
    #[error("sqlx error")]
    SqlxError(#[from] sqlx::Error),
    #[error("no such announcement.")]
    AnnouncementNotFound,
}

pub type Result<T> = std::result::Result<T, Error>;
