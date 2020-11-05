use diesel::pg::PgConnection;
use diesel::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbConnectionError {
    #[error("{0}")]
    ConnectionFailed(#[from] diesel::result::ConnectionError),
}

pub fn establish_connection(database_url: &str) -> Result<PgConnection, DbConnectionError> {
    PgConnection::establish(database_url).map_err(DbConnectionError::ConnectionFailed)
}
