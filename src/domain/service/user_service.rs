use crate::domain::model::{IUserRepositroy, User};
use std::sync::Arc;
use twitter2_api::error::ServiceError;

pub struct UserService {
    user_repository: Arc<dyn IUserRepositroy>,
}

impl UserService {
    pub fn new(user_repository: Arc<dyn IUserRepositroy>) -> Self {
        UserService { user_repository }
    }

    pub fn create<'a>(&self, name: &'a str) -> Result<User, ServiceError> {
        self.user_repository.create(name)
    }
}
