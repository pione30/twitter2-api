use crate::domain::model::UserId;
use crate::schema::users;
use chrono::{DateTime, Utc};
use diesel::{Insertable, QueryResult, Queryable};
use uuid::Uuid;

#[derive(Queryable, Debug, PartialEq, Eq)]
pub struct User {
    pub id: Uuid,
    pub sub_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug, PartialEq, Eq)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub sub_id: &'a str,
}

pub trait IUserRepository {
    fn create<'a>(&self, sub_id: &'a str) -> QueryResult<usize>;
    fn find_by_sub_id<'a>(&self, sub_id: &'a str) -> QueryResult<Option<User>>;
}
