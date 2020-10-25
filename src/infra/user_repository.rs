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
    fn create<'a>(&self, sub_id: &'a str) -> Result<User, ServiceError> {
        use crate::schema::users;

        let new_user = NewUser { sub_id };

        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(&*self.conn)
            .map_err(ServiceError::DbQueryFailed)
    }

    fn find_by_sub_id<'a>(&self, sub_id: &'a str) -> Result<User, ServiceError> {
        use crate::schema::users;

        users::table
            .filter(users::sub_id.eq(sub_id))
            .first(&*self.conn)
            .map_err(ServiceError::DbQueryFailed)
    }
}
