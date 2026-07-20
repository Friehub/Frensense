use serde::Deserialize;
use serde_json::Number;

#[derive(Deserialize)]
struct Data {
    #[serde(with = "serde_json::number")]
    id: Number,
}

fn main() {
    let json = r#"{"id": 9007199254740993}"#;
    let data: Data = serde_json::from_str(json).unwrap();
    // SAFE: `serde_json::Number` preserves the exact representation, avoiding f64 rounding.
    let precise: u64 = data.id.as_u64().unwrap();
    println!("{precise}");
}
