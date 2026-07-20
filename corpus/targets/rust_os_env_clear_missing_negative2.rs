// SAFE: Uses std::process::Command with a whitelist approach via remove_env to strip known sensitive variables selectively.

use std::process::Command;

fn run_worker(path: &str) -> Result<String, String> {
    let output = Command::new(path)
        .env_remove("AWS_SECRET_ACCESS_KEY")
        .env_remove("DATABASE_URL")
        .env_remove("API_TOKEN")
        .output()
        .map_err(|e| e.to_string())?;
    String::from_utf8(output.stdout).map_err(|e| e.to_string())
}
