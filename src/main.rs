mod domain;
mod infra;

mod app;
use app::App;

mod server;

use anyhow::Result;

#[tokio::main]
async fn main() {
    let server = server::Server::new();
    server.run(([0, 0, 0, 0], 3030)).await;
}
