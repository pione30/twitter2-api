mod infra;
use infra::establish_connection;
mod domain;
use anyhow::Result;
use std::io::stdin;

fn main() -> Result<()> {
    let connection = establish_connection()?;

    println!("Type the name of a new user.");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    // Drop the newline character
    let name = &name[..(name.len() - 1)];

    let user = infra::create(&connection, name)?;
    println!("Saved User with name: {}, id: {}", user.name, user.id);

    Ok(())
}
