mod app;
use app::App;

mod server;

use anyhow::Result;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let app = App::new(&database_url)?;

    let server = server::Server::new(app);
    server.run(([0, 0, 0, 0], 3030)).await;

    Ok(())
}
