// SAFE: Returns a Result with clear error messages, allowing the caller to handle missing variables gracefully
use std::env;

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub api_key: String,
    pub jwt_secret: String,
}

impl Config {
    fn from_env() -> Result<Self, String> {
        Ok(Config {
            database_url: env::var("DATABASE_URL").map_err(|_| "DATABASE_URL not set".to_string())?,
            api_key: env::var("API_KEY").map_err(|_| "API_KEY not set".to_string())?,
            jwt_secret: env::var("JWT_SECRET").map_err(|_| "JWT_SECRET not set".to_string())?,
        })
    }
}

fn main() -> Result<(), String> {
    let _cfg = Config::from_env()?;
    Ok(())
}
