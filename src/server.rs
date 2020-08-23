use std::future::Future;
use std::net::SocketAddr;
use warp::Filter;

pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Server {}
    }

    pub fn run<T: Into<SocketAddr> + 'static>(&self, addr: T) -> impl Future<Output = ()> {
        let routes = warp::path!("healthcheck").map(|| "ok");

        warp::serve(routes).run(addr)
    }
}
