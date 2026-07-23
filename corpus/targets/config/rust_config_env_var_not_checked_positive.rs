// [frensense]
// observation: `std::env::var("KEY")` is called and `.unwrap()` is used without checking for `Err`, causing a panic if the environment variable is not set.
// impact: The application crashes at startup if the required environment variable is missing, rather than providing a clear error message.
// improvement: Use `.expect("descriptive message")` or match on the `Result` to give a useful diagnostic.

use std::env;

fn load_config() -> String {
    let db_url = env::var("DATABASE_URL").unwrap();
    let api_key = env::var("API_KEY").unwrap();
    let secret = env::var("JWT_SECRET").unwrap();
    format!("{} {} {}", db_url, api_key, secret)
}

fn main() {
    let cfg = load_config();
    println!("{}", cfg);
}
