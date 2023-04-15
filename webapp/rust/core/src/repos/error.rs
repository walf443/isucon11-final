use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReposError {
    #[error("this is error for testing")]
    TestError,
    #[error("failed to execute query")]
    SqlError(#[from] sqlx::Error),
    #[error("A announcement with the same code already exists.")]
    AnnouncementDuplicate,
    #[error("A course with the same code already exists.")]
    CourseDuplicate,
    #[error("A class  with the same code already exists.")]
    ClassDuplicate,
}

pub type Result<T> = std::result::Result<T, ReposError>;
