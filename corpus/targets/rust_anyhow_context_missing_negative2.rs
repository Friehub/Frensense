// SAFE: Uses .with_context() with lazy format strings to provide detailed error context without allocation on success.

use anyhow::{Context, Result};
use std::fs;

fn load_config(path: &str) -> Result<String> {
    let data = fs::read_to_string(path)
        .with_context(|| format!("failed to open {}", path))?;
    Ok(data)
}
