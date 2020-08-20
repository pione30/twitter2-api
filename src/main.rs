mod domain;
mod infra;

mod app;
use app::App;

use anyhow::Result;

fn main() -> Result<()> {
    App::run()?;
    Ok(())
}
