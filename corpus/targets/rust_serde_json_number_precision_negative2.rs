use serde::Deserialize;

#[derive(Deserialize)]
struct Data {
    #[serde(deserialize_with = "deserialize_u64_from_json_number")]
    id: u64,
}

fn deserialize_u64_from_json_number<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let n = serde_json::Number::deserialize(deserializer)?;
    n.as_u64().ok_or_else(|| serde::de::Error::custom("expected u64"))
}

fn main() {
    let json = r#"{"id": 9007199254740993}"#;
    let data: Data = serde_json::from_str(json).unwrap();
    // SAFE: Deserializing through `serde_json::Number` then converting preserves exact value.
    println!("{}", data.id);
}
