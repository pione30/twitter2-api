use crate::domain::model::{IUserRepository, User};
use crate::error::ServiceError;
use std::sync::Arc;

pub struct UserService<UR>
where
    UR: IUserRepository,
{
    user_repository: Arc<UR>,
}

impl<UR> UserService<UR>
where
    UR: IUserRepository,
{
    pub fn new(user_repository: Arc<UR>) -> Self {
        UserService { user_repository }
    }

    pub fn create<'a>(&self, sub_id: &'a str) -> Result<User, ServiceError> {
        self.user_repository.create(sub_id)
    }
}
