use crate::domain::model::UserId;
use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable};
use twitter2_api::error::ServiceError;
use twitter2_api::schema::users;
use uuid::Uuid;

#[derive(Queryable, Debug, PartialEq, Eq)]
pub struct User {
    #[diesel(deserialize_as = "UserId")]
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug, PartialEq, Eq)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
}

pub trait IUserRepository {
    fn create<'a>(&self, name: &'a str) -> Result<User, ServiceError>;
    fn find_by_name<'a>(&self, name: &'a str) -> Result<User, ServiceError>;
}
