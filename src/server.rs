use crate::app::App;
use std::future::Future;
use std::net::SocketAddr;
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
    let api = warp::path("api");
    let v1 = warp::path("v1");
    let authorization = warp::header::<String>("authorization");

    let posts = warp::path("posts");
    let own_posts = warp::path("own").map(|| {
        let post_ids = vec![1, 2, 3, 42];
        warp::reply::json(&post_ids)
    });
    let posts = posts.and(own_posts);

    api.and(
        v1.and(authorization)
            .map(|token| {
                println!("Authorization token: {}", token);
            })
            // untuple_one() is necessary
            .untuple_one()
            .and(posts),
    )
    .boxed()
}
