use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("{0}")]
    DbQueryFailed(#[from] diesel::result::Error),
}
