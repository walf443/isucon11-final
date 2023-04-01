use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReposError {
    #[error("failed to execute query")]
    SqlError(#[from] sqlx::Error),
}

pub type Result<T> = std::result::Result<T, ReposError>;
