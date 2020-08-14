use crate::domain::model::{IUserRepositroy, NewUser, User};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::sync::Arc;
use twitter2_api::error::ServiceError;

pub struct UserRepository {
    conn: Arc<PgConnection>,
}

impl UserRepository {
    pub fn new(conn: Arc<PgConnection>) -> Self {
        UserRepository { conn }
    }
}

impl IUserRepositroy for UserRepository {
    fn create<'a>(&self, name: &'a str) -> Result<User, ServiceError> {
        use twitter2_api::schema::users;

        let new_user = NewUser { name };

        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(&*self.conn)
            .map_err(ServiceError::CreationFailed)
    }
}
