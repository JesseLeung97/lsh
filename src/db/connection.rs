extern crate redis;
use redis::RedisResult;

pub struct DBConnection {
    pub client: redis::Client,
    pub connection: redis::Connection
}