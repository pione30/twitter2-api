use crate::domain::model::{IPostRepository, NewPost, Post, User};
use crate::error::ServiceError;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::sync::Arc;

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
        use crate::schema::posts;

        let new_post = NewPost {
            body,
            user_id: &user.id,
        };

        diesel::insert_into(posts::table)
            .values(&new_post)
            .get_result(&*self.conn)
            .map_err(ServiceError::CreationFailed)
    }

    fn pagenate_posts_of_user<'a>(
        &self,
        user: &'a User,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Post>, ServiceError> {
        use crate::schema::posts;
        use crate::schema::users;

        posts::table
            .inner_join(users::table.on(users::id.eq(user.id)))
            .select(posts::all_columns)
            .order_by(posts::created_at.desc())
            .limit(limit)
            .offset(offset)
            .load(&*self.conn)
            .map_err(ServiceError::DBError)
    }
}
