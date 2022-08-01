use redis::{Commands, RedisError, RedisResult};
use crate::connection::DBConnection;
use crate::vector::Vector;

/// ### Database structure
/// ### Query flow
/// Query user hash table by user_id to find hashcode
/// 
/// Query each hash table to find users in same bucket as hashcode
pub struct Database {}
impl  Database {
    pub fn connect(&self, connection_url: String) -> DBConnection {
        let client = redis::Client::open(connection_url);
        let client = match client {
            Ok(client) => client,
            Err(_) => panic!("Could not open the client.")
        };
        let connection = client.get_connection();
        let connection = match connection {
            Ok(con) => con,
            Err(_) => panic!("Could not connect to the database")
        };
        
        DBConnection { client, connection }
    }
    
    pub fn get_user_hashcodes(&self, db_connection: &mut DBConnection, user_id: u32) -> Vec<String> {
        let user_id_str = user_id.to_string();
        let result = db_connection.connection.hgetall(user_id_str);
        let result = match result {
            Ok(&mut val) => val.split(",").collect(),
            Err(_) => panic!("There was no user found with the ID: {}", user_id.to_string()),
        };
        
        result
    }
    
    pub fn add_user(&self, db_connection: &mut DBConnection, user_id: u32, user_hashcodes: Vec<u32>) {
        let user_id_str = user_id.to_string();
        let hashcodes = Self::to_value_string(&user_hashcodes);
        let result: RedisResult<i32> = db_connection.connection.hset(user_id_str, "table_1", hashcodes);
        let _result = match result {
            Ok(_) => println!("Successfully wrote user to table."),
            Err(_) => panic!("Could not write the user to the table.")
        };
    }
    
    fn to_value_string(user_hashcodes: &Vec<u32>) -> String {
        user_hashcodes.iter().fold("".to_owned(), |acc, hashcode| format!("{},{}", acc, hashcode))
    }
    // 
    // pub fn get_matching_users(db_connection: &mut DBConnection, user_hashcodes: Vec<u32>) -> Vec<Vector> {
    // }
}