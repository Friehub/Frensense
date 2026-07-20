use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    role: String,
    #[serde(flatten)]
    extra: HashMap<String, String>,
}

fn main() {
    let json = r#"{"role":"user","debug":"true"}"#;
    // SAFE: deny_unknown_fields ensures duplicate keys and unexpected fields are rejected.
    let config: Config = serde_json::from_str(json).unwrap();
    println!("role = {}", config.role);
}
