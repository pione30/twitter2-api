use crate::domain::model::{IPostRepository, IUserRepository, Post};
use crate::error::ServiceError;
use std::sync::Arc;

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

    pub fn create<'a>(&self, body: &'a str, user_name: &'a str) -> Result<Post, ServiceError> {
        let user = self.user_repository.find_by_name(user_name)?;
        self.post_repository.create(body, &user)
    }
}
