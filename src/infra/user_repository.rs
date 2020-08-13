use crate::domain::model::{NewUser, User};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserRepositoryError {
    #[error("{0}")]
    CreationFailed(#[from] diesel::result::Error),
}

pub fn create<'a>(conn: &PgConnection, name: &'a str) -> Result<User, UserRepositoryError> {
    use twitter2_api::schema::users;

    let new_user = NewUser { name };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .map_err(UserRepositoryError::CreationFailed)
}
