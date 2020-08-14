mod domain;
mod infra;
use anyhow::Result;
use std::io::stdin;
use std::sync::Arc;

fn main() -> Result<()> {
    let connection = Arc::new(infra::db_connector::establish_connection()?);

    println!("Type the name of a new user.");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    // Drop the newline character
    let name = &name[..(name.len() - 1)];

    use domain::model::IUserRepositroy;
    let conn = Arc::clone(&connection);
    let user_repository = infra::UserRepository::new(conn);
    let user = user_repository.create(name)?;

    println!("Saved User with name: {}, id: {}", user.name, user.id);

    let user = user_repository.find_by_name(name)?;
    println!("That user is just now fetched as: {:?}", user);

    Ok(())
}
