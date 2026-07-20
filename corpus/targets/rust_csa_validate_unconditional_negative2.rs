// SAFE: Returns a Result type, propagating validation failures as Err
use std::collections::HashMap;

#[derive(Debug)]
enum ValidationError {
    EmptyHost,
    InvalidPort,
    InvalidTimeout,
    TooManyRetries,
    TlsRequired,
}

fn validate_config(input: &Config) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    if input.host.is_empty() {
        errors.push(ValidationError::EmptyHost);
    }

    if input.port == 0 || input.port > 65535 {
        errors.push(ValidationError::InvalidPort);
    }

    if input.timeout_ms == 0 || input.timeout_ms > 300_000 {
        errors.push(ValidationError::InvalidTimeout);
    }

    if input.max_retries > 10 {
        errors.push(ValidationError::TooManyRetries);
    }

    if !input.tls_enabled {
        errors.push(ValidationError::TlsRequired);
    }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}
