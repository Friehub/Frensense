use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    command: String,
}

fn load_config(yaml: &str) -> Result<Config, serde_yaml::Error> {
    // SAFE: Use `from_str` on a struct that strictly types all fields.
    // The `String` type already rejects non-string YAML tags in serde_yaml.
    serde_yaml::from_str(yaml)
}

fn main() {
    let safe = "command: ls -la";
    let config = load_config(safe).unwrap();
    println!("{:?}", config);
}
