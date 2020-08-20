use crate::domain::model::{IPostRepository, IUserRepositroy, Post};
use std::sync::Arc;
use twitter2_api::error::ServiceError;

pub struct PostService {
    post_repository: Arc<dyn IPostRepository>,
    user_repository: Arc<dyn IUserRepositroy>,
}

impl PostService {
    pub fn new(
        post_repository: Arc<dyn IPostRepository>,
        user_repository: Arc<dyn IUserRepositroy>,
    ) -> Self {
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
