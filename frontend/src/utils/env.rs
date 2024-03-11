use std::env;

pub fn get_env(key: &str) -> String {
    dotenv::dotenv().ok();
    env::var(key).unwrap()
}
