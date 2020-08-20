use crate::domain::model::{IUserRepositroy, User};
use std::sync::Arc;
use twitter2_api::error::ServiceError;

pub struct UserService<UR>
where
    UR: IUserRepositroy,
{
    user_repository: Arc<UR>,
}

impl<UR> UserService<UR>
where
    UR: IUserRepositroy,
{
    pub fn new(user_repository: Arc<UR>) -> Self {
        UserService { user_repository }
    }

    pub fn create<'a>(&self, name: &'a str) -> Result<User, ServiceError> {
        self.user_repository.create(name)
    }
}
