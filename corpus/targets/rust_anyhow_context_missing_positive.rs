// [frensense]
// observation: An anyhow::Result function uses the ? operator without calling .context() or .with_context(), losing the semantic meaning of the error when it propagates.
// impact: When an error propagates to the user or log, the error message only shows the low-level cause (e.g., "PermissionDenied") without any context about what operation failed (e.g., "failed to open config file at /etc/app/config.toml").
// improvement: Use .context() or .with_context() on every ? in fallible operations to attach human-readable context.

use anyhow::Result;
use std::fs;

fn load_config(path: &str) -> Result<String> {
    let data = fs::read_to_string(path)?;
    Ok(data)
}

fn parse_and_validate(path: &str) -> Result<serde_json::Value> {
    let text = load_config(path)?;
    let value: serde_json::Value = serde_json::from_str(&text)?;
    Ok(value)
}
