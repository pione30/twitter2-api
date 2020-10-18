use crate::app::App;
use std::env;
use std::future::Future;
use std::net::SocketAddr;
use twitter2_api::infra::jwt_handler;
use warp::{filters::BoxedFilter, Filter, Reply};

pub struct Server {
    app: App,
}

impl Server {
    pub fn new(app: App) -> Self {
        Server { app }
    }

    pub fn run<T: Into<SocketAddr> + 'static>(&self, addr: T) -> impl Future<Output = ()> {
        let routes = healthcheck().or(api());

        warp::serve(routes).run(addr)
    }
}

fn healthcheck() -> BoxedFilter<(impl Reply,)> {
    warp::path("healthcheck").map(|| "ok").boxed()
}

fn api() -> BoxedFilter<(impl Reply,)> {
    let allowed_origin = env::var("ALLOWED_ORIGIN").expect("ALLOWED_ORIGIN must be set");
    let cors = warp::cors()
        .allow_origin(allowed_origin.as_str())
        .allow_headers(vec!["authorization"])
        .allow_methods(vec!["GET", "POST", "DELETE"]);

    let authorization = warp::header::<String>("authorization")
        .and_then(|autorization_token: String| async move {
            let token = autorization_token
                .trim()
                .strip_prefix("Bearer ")
                .ok_or_else(warp::reject::reject)?;

            jwt_handler::verify(token)
                .await
                .map(|claim| {
                    println!("{:?}", claim);
                })
                .map_err(|err| {
                    println!("{:?}", err);
                    warp::reject::reject()
                })
        })
        // untuple_one() is necessary
        .untuple_one();

    let api = warp::path("api");
    let v1 = warp::path("v1");

    let posts = warp::path("posts");
    let own_posts = warp::path("own").map(|| {
        let post_ids = vec![1, 2, 3, 42];
        warp::reply::json(&post_ids)
    });
    let posts = posts.and(own_posts);

    api.and(v1.and(authorization).and(posts)).with(cors).boxed()
}
