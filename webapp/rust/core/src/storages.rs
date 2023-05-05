use thiserror::Error;

pub mod submission_file_storage;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("io error")]
    IOError(#[from] std::io::Error),
}

pub type StorageResult<T> = Result<T, StorageError>;
