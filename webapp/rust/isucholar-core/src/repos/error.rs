use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReposError {
    #[error("failed to execute query")]
    SqlError(#[from] sqlx::Error),
    #[error("A announcement with the same code already exists.")]
    AnnoucementDuplicate,
    #[error("A course with the same code already exists.")]
    CourseDepulicate,
    #[error("A class  with the same code already exists.")]
    ClassDepulicate,
}

pub type Result<T> = std::result::Result<T, ReposError>;
