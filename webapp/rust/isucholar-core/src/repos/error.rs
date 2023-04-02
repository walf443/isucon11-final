use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReposError {
    #[error("failed to execute query")]
    SqlError(#[from] sqlx::Error),
    #[error("A course with the same code already exists.")]
    CourseDepulicate(),
}

pub type Result<T> = std::result::Result<T, ReposError>;
