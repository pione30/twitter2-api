mod domain;
mod infra;
use anyhow::Result;
use std::io::stdin;

fn main() -> Result<()> {
    let connection = infra::db_connector::establish_connection()?;

    println!("Type the name of a new user.");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    // Drop the newline character
    let name = &name[..(name.len() - 1)];

    let user = infra::user_repository::create(&connection, name)?;
    println!("Saved User with name: {}, id: {}", user.name, user.id);

    Ok(())
}
