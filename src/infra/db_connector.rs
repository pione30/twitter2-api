use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbConnectionError {
    #[error("{0}")]
    Connection(#[from] diesel::ConnectionError),
}

pub fn establish_connection() -> Result<PgConnection, DbConnectionError> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).map_err(DbConnectionError::Connection)
}
