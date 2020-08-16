use crate::domain::model::{IPostRepository, NewPost, Post, User};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::sync::Arc;
use twitter2_api::error::ServiceError;

pub struct PostRepository {
    conn: Arc<PgConnection>,
}

impl PostRepository {
    pub fn new(conn: Arc<PgConnection>) -> Self {
        PostRepository { conn }
    }
}

impl IPostRepository for PostRepository {
    fn create<'a>(&self, body: &'a str, user: &'a User) -> Result<Post, ServiceError> {
        use twitter2_api::schema::posts;

        let new_post = NewPost {
            body,
            user_id: &user.id,
        };

        diesel::insert_into(posts::table)
            .values(&new_post)
            .get_result(&*self.conn)
            .map_err(ServiceError::CreationFailed)
    }
}
