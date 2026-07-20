use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    #[serde(deserialize_with = "deserialize_untagged_string")]
    command: String,
}

fn deserialize_untagged_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    // SAFE: Accept only plain strings; reject tagged YAML values.
    let value = serde_yaml::Value::deserialize(deserializer)?;
    match value {
        serde_yaml::Value::String(s) => Ok(s),
        _ => Err(serde::de::Error::custom("expected plain string")),
    }
}

fn load_config(yaml: &str) -> Result<Config, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

fn main() {
    let malicious = "command: id";
    let config = load_config(malicious).unwrap();
    println!("{:?}", config);
}
