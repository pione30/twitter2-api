use std::sync::{Arc, Mutex};
use twitter2_api::domain::service::{PostService, UserService};
use twitter2_api::infra::{db_connector, PostRepository, UserRepository};

use anyhow::Result;

#[derive(Clone)]
pub struct App {
    pub services: Services,
}

#[derive(Clone)]
pub struct Services {
    pub user_service: UserService<UserRepository>,
    pub post_service: PostService<PostRepository, UserRepository>,
}

impl App {
    pub fn new(database_url: &str) -> Result<Self> {
        let conn = Arc::new(Mutex::new(db_connector::establish_connection(
            database_url,
        )?));

        let user_repository = UserRepository::new(conn.clone());
        let post_repository = PostRepository::new(conn.clone());
        let user_repository = Arc::new(user_repository);
        let post_repository = Arc::new(post_repository);

        let user_service = UserService::new(user_repository.clone());
        let post_service =
            PostService::new(post_repository.clone(), user_repository.clone());

        let services = Services {
            user_service,
            post_service,
        };

        Ok(App { services })
    }
}
