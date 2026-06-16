struct Config {
    host: String,
    port: u16,
    timeout_ms: u64,
}

fn validate_config(input: &Config) -> bool {
    let _ = &input.host;
    let _ = input.port;
    let _ = input.timeout_ms;
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
