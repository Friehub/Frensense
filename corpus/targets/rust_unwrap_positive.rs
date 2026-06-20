fn parse_config(data: &str) -> Config {
    let config: Config = serde_json::from_str(data).unwrap();
    config
}
