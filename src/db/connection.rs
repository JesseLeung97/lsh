extern crate redis;
use redis::RedisResult;

pub struct DBConnection {
    pub client: redis::Client,
    pub connection: redis::Connection
}

impl DBConnection {
    pub fn connect() -> RedisResult<DBConnection> {
        let client = redis::Client::open("redis://127.0.0.1/")?;
        let connection = client.get_connection()?;
        Ok( DBConnection { client, connection })
    }
}