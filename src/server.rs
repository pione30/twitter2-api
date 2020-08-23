use crate::app::App;
use std::future::Future;
use std::net::SocketAddr;
use warp::Filter;

pub struct Server {
    app: App,
}

impl Server {
    pub fn new(app: App) -> Self {
        Server { app }
    }

    pub fn run<T: Into<SocketAddr> + 'static>(&self, addr: T) -> impl Future<Output = ()> {
        let routes = warp::path!("healthcheck").map(|| "ok");

        warp::serve(routes).run(addr)
    }
}
