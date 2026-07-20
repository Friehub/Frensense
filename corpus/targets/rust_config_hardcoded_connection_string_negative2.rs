// SAFE: Uses a config file (TOML) loaded at runtime instead of hardcoded credentials
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct AppConfig {
    database_url: String,
    redis_url: String,
}

fn load_config(path: &str) -> Result<AppConfig, String> {
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let config: AppConfig = toml::from_str(&content).map_err(|e| e.to_string())?;
    Ok(config)
}

async fn connect_db() -> Result<sqlx::PgPool, String> {
    let config = load_config("/etc/app/config.toml")?;
    let pool = sqlx::PgPool::connect(&config.database_url).await.map_err(|e| e.to_string())?;
    Ok(pool)
}
