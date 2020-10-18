use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("{0}")]
    CreationFailed(diesel::result::Error),
    #[error("{0}")]
    LoadFromDBFaild(diesel::result::Error),
    #[error("{0}")]
    DBError(#[from] diesel::result::Error),
}
