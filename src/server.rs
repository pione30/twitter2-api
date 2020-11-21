use crate::app::App;
use std::collections::HashMap;
use std::env;
use std::future::Future;
use std::net::SocketAddr;
use twitter2_api::infra::jwt_handler;
use warp::{filters::BoxedFilter, http::status::StatusCode, Filter, Reply};

pub struct Server {
    app: App,
}

impl Server {
    pub fn new(app: App) -> Self {
        Server { app }
    }

    pub fn run<T: Into<SocketAddr> + 'static>(&self, addr: T) -> impl Future<Output = ()> {
        let routes = self.healthcheck().or(self.api());

        warp::serve(routes).run(addr)
    }

    fn healthcheck(&self) -> BoxedFilter<(String,)> {
        warp::path("healthcheck").map(|| "ok".into()).boxed()
    }

    fn api(&self) -> BoxedFilter<(impl Reply,)> {
        let allowed_origin = env::var("ALLOWED_ORIGIN").expect("ALLOWED_ORIGIN must be set");
        let cors = warp::cors()
            .allow_origin(allowed_origin.as_str())
            .allow_headers(vec!["authorization"])
            .allow_methods(vec!["GET", "POST", "PUT", "DELETE"]);

        let authorization = warp::header::<String>("authorization").and_then(
            |autorization_token: String| async move {
                let token = autorization_token
                    .trim()
                    .strip_prefix("Bearer ")
                    .ok_or_else(warp::reject::reject)?;

                jwt_handler::verify(token).await.map_err(|err| {
                    eprintln!("{:?}", err);
                    warp::reject::reject()
                })
            },
        );

        let api = warp::path("api");
        let v1 = warp::path("v1");

        let app = self.app.clone();
        let posts = warp::path("posts");
        let own_posts =
            warp::path("own")
                .and(authorization)
                .map(move |claim: jwt_handler::Claims| {
                    app.services
                        .post_service
                        .pagenate_posts_of_user_by_sub_id(&claim.sub, 20, 0)
                        .map_or_else(
                            |err| {
                                let mut data = HashMap::new();
                                data.insert("error".to_string(), format!("{}", err));
                                warp::reply::with_status(
                                    warp::reply::json(&data),
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                )
                            },
                            |own_posts| {
                                warp::reply::with_status(
                                    warp::reply::json(&own_posts),
                                    StatusCode::OK,
                                )
                            },
                        )
                });
        let posts = posts.and(own_posts);

        api.and(v1.and(posts)).with(cors).boxed()
    }
}
