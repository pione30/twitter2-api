use crate::domain::service::{PostService, UserService};
use crate::infra::{db_connector, PostRepository, UserRepository};
use std::sync::Arc;

use anyhow::Result;

pub struct App {}

impl App {
    pub fn run() -> Result<()> {
        let conn = Arc::new(db_connector::establish_connection()?);
        let user_repository = UserRepository::new(Arc::clone(&conn));
        let post_repository = PostRepository::new(Arc::clone(&conn));
        let user_repository = Arc::new(user_repository);
        let post_repository = Arc::new(post_repository);

        let user_service = UserService::new(Arc::clone(&user_repository));
        let post_service =
            PostService::new(Arc::clone(&post_repository), Arc::clone(&user_repository));
        Ok(())
    }
}
