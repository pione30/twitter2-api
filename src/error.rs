use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Not Found")]
    NotFound,
    #[error("{0}")]
    DbQueryFailed(#[from] diesel::result::Error),
}
