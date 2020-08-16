use crate::domain::model::{PostId, User, UserId};
use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable};
use twitter2_api::error::ServiceError;
use twitter2_api::schema::posts;
use uuid::Uuid;

#[derive(Queryable, Debug, PartialEq, Eq)]
pub struct Post {
    #[diesel(deserialize_as = "PostId")]
    pub id: Uuid,
    pub body: String,
    #[diesel(deserialize_as = "UserId")]
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Debug, PartialEq, Eq)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub body: &'a str,
    pub user_id: &'a Uuid,
}

pub trait IPostRepository {
    fn create<'a>(&self, body: &'a str, user: &'a User) -> Result<Post, ServiceError>;
}
