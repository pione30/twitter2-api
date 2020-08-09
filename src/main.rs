mod infra;
use infra::establish_connection;

fn main() {
    let connection = establish_connection();
}
