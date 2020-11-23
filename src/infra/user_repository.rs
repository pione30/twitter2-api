use crate::domain::model::{IUserRepository, NewUser, User};
use crate::error::ServiceError;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct UserRepository {
    conn: Arc<Mutex<PgConnection>>,
}

impl UserRepository {
    pub fn new(conn: Arc<Mutex<PgConnection>>) -> Self {
        UserRepository { conn }
    }
}

impl IUserRepository for UserRepository {
    fn create<'a>(&self, sub_id: &'a str) -> Result<User, ServiceError> {
        use crate::schema::users;

        let new_user = NewUser { sub_id };

        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(&*self.conn.lock().unwrap())
            .map_err(ServiceError::DbQueryFailed)
    }

    fn find_by_sub_id<'a>(&self, sub_id: &'a str) -> Result<Option<User>, ServiceError> {
        use crate::schema::users;

        users::table
            .filter(users::sub_id.eq(sub_id))
            .first(&*self.conn.lock().unwrap())
            .optional()
            .map_err(ServiceError::DbQueryFailed)
    }
}
