struct Config {
    host: String,
    port: u16,
    timeout_ms: u64,
}

fn validate_config(input: &Config) -> bool {
    if input.host.is_empty() {
        return false;
    }
    if input.port == 0 {
        return false;
    }
    if input.timeout_ms == 0 || input.timeout_ms > 30_000 {
        return false;
    }
    true
}

fn process_request(cfg: &Config) -> Result<String, Box<dyn std::error::Error>> {
    if !validate_config(cfg) {
        return Err("invalid config".into());
    }
    let url = format!("http://{}:{}", cfg.host, cfg.port);
    let body = reqwest::blocking::get(&url)?.text()?;
    Ok(body)
}
