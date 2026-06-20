fn parse_config(data: &str) -> Result<Config, String> {
    let config: Config = serde_json::from_str(data)
        .map_err(|e| format!("Failed to parse config: {}", e))?;
    Ok(config)
}
