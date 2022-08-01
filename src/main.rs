use crate::connection::DBConnection;
use crate::database::Database;

mod lsh;
mod config;
mod vector;
#[path = "db/connection.rs"] mod connection;
#[path = "db/database.rs"] mod database;

fn main() {
    println!("Running LSH main...");
    
    let user_db = Database {};
    let mut user_db_connection = user_db.connect("redis://127.0.0.1:6379".to_owned());
    user_db.add_user(&mut user_db_connection, 12345, vec![123,456]);
    let res = user_db.get_user_hashcodes(&mut user_db_connection, 12345);
}

