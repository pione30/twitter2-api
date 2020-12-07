use crate::domain::model::{IPostRepository, IUserRepository, Post};
use crate::error::ServiceError;
use std::sync::Arc;

#[derive(Clone)]
pub struct PostService<PR, UR>
where
    PR: IPostRepository,
    UR: IUserRepository,
{
    post_repository: Arc<PR>,
    user_repository: Arc<UR>,
}

impl<PR, UR> PostService<PR, UR>
where
    PR: IPostRepository,
    UR: IUserRepository,
{
    pub fn new(post_repository: Arc<PR>, user_repository: Arc<UR>) -> Self {
        PostService {
            post_repository,
            user_repository,
        }
    }

    pub fn create<'a>(&self, body: &'a str, sub_id: &'a str) -> Result<Post, ServiceError> {
        let user = self
            .user_repository
            .find_by_sub_id(sub_id)?
            .ok_or(ServiceError::NotFound)?;

        self.post_repository
            .create(body, &user)
            .map_err(ServiceError::DbQueryFailed)
    }

    pub fn pagenate_posts_of_user_by_sub_id(
        &self,
        sub_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Post>, ServiceError> {
        let user = self
            .user_repository
            .find_by_sub_id(sub_id)?
            .ok_or(ServiceError::NotFound)?;

        self.post_repository
            .pagenate_posts_of_user(&user, limit, offset)
            .map_err(ServiceError::DbQueryFailed)
    }
}
