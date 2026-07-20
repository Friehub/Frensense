// SAFE: Every fallible operation uses .context() to attach semantic context to errors.

use anyhow::{Context, Result};
use std::fs;

fn load_config(path: &str) -> Result<String> {
    let data = fs::read_to_string(path)
        .with_context(|| format!("failed to read config file at {}", path))?;
    Ok(data)
}

fn parse_and_validate(path: &str) -> Result<serde_json::Value> {
    let text = load_config(path)?;
    let value: serde_json::Value = serde_json::from_str(&text)
        .context("failed to parse config as JSON")?;
    Ok(value)
}
