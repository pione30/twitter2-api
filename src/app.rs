use std::sync::Arc;
use twitter2_api::domain::service::{PostService, UserService};
use twitter2_api::infra::{db_connector, PostRepository, UserRepository};

use anyhow::Result;

pub struct App {
    pub services: Services,
}

pub struct Services {
    pub user_service: UserService<UserRepository>,
    pub post_service: PostService<PostRepository, UserRepository>,
}

impl App {
    pub fn new(database_url: &str) -> Result<Self> {
        let conn = Arc::new(db_connector::establish_connection(database_url)?);
        let user_repository = UserRepository::new(Arc::clone(&conn));
        let post_repository = PostRepository::new(Arc::clone(&conn));
        let user_repository = Arc::new(user_repository);
        let post_repository = Arc::new(post_repository);

        let user_service = UserService::new(Arc::clone(&user_repository));
        let post_service =
            PostService::new(Arc::clone(&post_repository), Arc::clone(&user_repository));

        let services = Services {
            user_service,
            post_service,
        };

        Ok(App { services })
    }
}
