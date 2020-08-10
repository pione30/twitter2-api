mod infra;
use infra::establish_connection;

use anyhow::Result;

fn main() -> Result<()> {
    let connection = establish_connection()?;

    Ok(())
}
