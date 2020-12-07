use crate::domain::model::{PostId, User, UserId};
use crate::schema::posts;
use chrono::{DateTime, Utc};
use diesel::{Insertable, QueryResult, Queryable};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Queryable, Debug, PartialEq, Eq)]
pub struct Post {
    pub id: Uuid,
    pub body: String,
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
    fn create<'a>(&self, body: &'a str, user: &'a User) -> QueryResult<Post>;
    fn pagenate_posts_of_user<'a>(
        &self,
        user: &'a User,
        limit: i64,
        offset: i64,
    ) -> QueryResult<Vec<Post>>;
}
