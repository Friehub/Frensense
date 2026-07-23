// SAFE: Uses `.expect()` with descriptive messages so failures are informative
use std::env;

fn load_config() -> String {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    format!("{} {} {}", db_url, api_key, secret)
}

fn main() {
    let cfg = load_config();
    println!("{}", cfg);
}
