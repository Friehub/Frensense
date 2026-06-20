// Rule: RSA_CSA_VALIDATE_UNCONDITIONAL (negative — no rule expected)
// A function that properly validates config and rejects invalid values.

use std::collections::HashMap;

struct Config {
    host: String,
    port: u16,
    timeout_ms: u64,
    max_retries: u32,
    tls_enabled: bool,
}

struct ConfigValidation {
    valid: bool,
    errors: Vec<String>,
}

fn validate_config(input: &Config) -> ConfigValidation {
    let mut errors = Vec::new();

    // Validate host
    if input.host.is_empty() {
        errors.push("host cannot be empty".to_string());
    } else if input.host.len() > 253 {
        errors.push("host is too long (max 253 characters)".to_string());
    } else if !input.host.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '-' || c == ':') {
        errors.push("host contains invalid characters".to_string());
    }

    // Validate port
    if input.port == 0 {
        errors.push("port must be non-zero".to_string());
    } else if input.port > 65535 {
        errors.push("port must be between 1 and 65535".to_string());
    }

    // Validate timeout
    if input.timeout_ms == 0 {
        errors.push("timeout must be positive".to_string());
    } else if input.timeout_ms > 300_000 {
        errors.push("timeout cannot exceed 5 minutes (300000ms)".to_string());
    }

    // Validate retries
    if input.max_retries > 10 {
        errors.push("max_retries cannot exceed 10".to_string());
    }

    // TLS is required for production
    if !input.tls_enabled {
        errors.push("tls_enabled must be true for production use".to_string());
    }

    ConfigValidation {
        valid: errors.is_empty(),
        errors,
    }
}

fn process_request(cfg: &Config) -> Result<String, Box<dyn std::error::Error>> {
    let validation = validate_config(cfg);
    if !validation.valid {
        let error_msg = validation.errors.join("; ");
        return Err(format!("Config validation failed: {}", error_msg).into());
    }

    let url = format!("https://{}:{}", cfg.host, cfg.port);
    let body = reqwest::blocking::get(&url)?.text()?;
    Ok(body)
}
