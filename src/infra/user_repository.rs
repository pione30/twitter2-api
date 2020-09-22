use crate::domain::model::{IUserRepository, NewUser, User};
use crate::error::ServiceError;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::sync::Arc;

pub struct UserRepository {
    conn: Arc<PgConnection>,
}

impl UserRepository {
    pub fn new(conn: Arc<PgConnection>) -> Self {
        UserRepository { conn }
    }
}

impl IUserRepository for UserRepository {
    fn create<'a>(&self, name: &'a str) -> Result<User, ServiceError> {
        use crate::schema::users;

        let new_user = NewUser { name };

        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(&*self.conn)
            .map_err(ServiceError::CreationFailed)
    }

    fn find_by_name<'a>(&self, name: &'a str) -> Result<User, ServiceError> {
        use crate::schema::users;

        users::table
            .filter(users::name.eq(name))
            .first(&*self.conn)
            .map_err(ServiceError::LoadFromDBFaild)
    }
}
