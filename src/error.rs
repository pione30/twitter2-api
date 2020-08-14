use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("{0}")]
    CreationFailed(#[from] diesel::result::Error),
}
