use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable};
use twitter2_api::schema::posts;
use uuid::Uuid;

#[derive(Queryable, Debug, PartialEq, Eq)]
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
