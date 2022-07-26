use redis::Commands;
use crate::connection::DBConnection;
use crate::vector::Vector;

/// ### Database structure
/// ### Query flow
/// Query user hash table by user_id to find hashcode
/// 
/// Query each hash table to find users in same bucket as hashcode
pub struct Database {}
impl  Database {
    pub fn get_user_hashcodes(db_connection: &mut DBConnection, user_id: u32) -> Vec<u32> {
        let user_id_str = user_id.to_string();
        let result = db_connection.connection.hgetall(user_id_str);
        let result = match result {
            Ok(val) => vec!(val) as Vec<u32>,
            Err(_) => panic!("There was no user found with the ID: {}", user_id.to_string()),
        };
        
        result
    }
    
    pub fn get_matching_users(db_connection: &mut DBConnection, user_hashcodes: Vec<u32>) -> Vec<Vector> {
    }
}