mod domain;
mod infra;

use anyhow::Result;
use domain::model::{IPostRepository, IUserRepositroy};
use infra::{db_connector, PostRepository, UserRepository};
use std::io::stdin;
use std::sync::Arc;

fn main() -> Result<()> {
    let connection = Arc::new(db_connector::establish_connection()?);

    println!("Type the name of a new user:");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    // Drop the newline character
    let name = &name[..(name.len() - 1)];

    let conn_user_repo = Arc::clone(&connection);
    let user_repository = UserRepository::new(conn_user_repo);
    let user = user_repository.create(name)?;

    println!("Saved User with name: {}, id: {}", user.name, user.id);

    let user = user_repository.find_by_name(name)?;
    println!("That user is just now fetched as: {:?}", user);

    println!("------------------");

    println!("Write some body of the sentence:");
    let mut body = String::new();
    stdin().read_line(&mut body).unwrap();
    // Drop the newline character
    let body = &body[..(body.len() - 1)];

    let conn_post_repo = Arc::clone(&connection);
    let post_repository = PostRepository::new(conn_post_repo);
    let post = post_repository.create(body, &user)?;

    println!(
        "Saved the user {}'s new post with body: {}",
        user.name, post.body
    );

    Ok(())
}
