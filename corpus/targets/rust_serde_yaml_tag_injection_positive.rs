// [frensense]
// observation: `serde_yaml::from_str` is called on untrusted YAML input without restricting YAML tags. YAML tags like `!!python/object:os.system` can trigger arbitrary code execution in certain YAML libraries, and `!!str`/`!!seq` tags can bypass type validation logic in `serde_yaml`.
// impact: An attacker can craft a YAML payload that executes arbitrary commands or bypasses type checks via YAML tags. This is particularly dangerous when deserializing YAML from external sources (API bodies, config files from users).
// improvement: Use `serde_yaml::Value` with tag restrictions, switch to JSON/MessagePack for untrusted input, or validate after deserialization.

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    command: String,
}

fn load_config(yaml: &str) -> Result<Config, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

fn main() {
    let malicious = "command: !!str id";
    let config = load_config(malicious).unwrap();
    println!("{:?}", config);
}
