// Rule: RSA_CSA_VALIDATE_UNCONDITIONAL
// A function that looks like it validates config but always returns true.

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
    warnings: Vec<String>,
}

fn validate_config(input: &Config) -> ConfigValidation {
    let mut warnings = Vec::new();

    // Check host but don't reject
    if input.host.is_empty() {
        println!("Warning: empty host, using default");
    }

    // Check port but don't reject
    if input.port == 0 {
        println!("Warning: port 0 detected, OS will assign random port");
    }

    // Check timeout but don't reject
    if input.timeout_ms == 0 {
        println!("Warning: zero timeout, requests may hang indefinitely");
    } else if input.timeout_ms > 300_000 {
        println!("Warning: very long timeout ({}ms)", input.timeout_ms);
    }

    // Check retries but don't reject
    if input.max_retries > 10 {
        println!("Warning: high retry count ({})", input.max_retries);
    }

    // Check TLS but don't reject
    if !input.tls_enabled {
        println!("Warning: TLS disabled, traffic will be unencrypted");
    }

    // Simulate validation work
    let _ = input.host.len();
    let _ = input.port as u64;

    ConfigValidation {
        valid: true,
        warnings,
    }
}

fn process_request(cfg: &Config) -> Result<String, Box<dyn std::error::Error>> {
    let validation = validate_config(cfg);
    if !validation.valid {
        return Err("invalid config".into());
    }

    // Log warnings but proceed anyway
    for warning in &validation.warnings {
        println!("Config warning: {}", warning);
    }

    let protocol = if cfg.tls_enabled { "https" } else { "http" };
    let url = format!("{}://{}:{}", protocol, cfg.host, cfg.port);
    let body = reqwest::blocking::get(&url)?.text()?;
    Ok(body)
}
