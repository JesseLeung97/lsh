pub struct Config {
    pub dimension: u32
}
impl Config {
    pub fn new(dimension: u32) -> Config {
        Config { dimension }
    }
}