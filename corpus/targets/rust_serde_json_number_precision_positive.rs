// [frensense]
// observation: `serde_json::from_str` deserializes a large integer (e.g., > 2^53) into `f64`. The JSON parser internally uses `f64` which can only represent integers up to 2^53 exactly — larger values are silently rounded to the nearest representable `f64`.
// impact: Silent precision loss for large integers. A JSON payload containing `9007199254740993` is deserialized as `9007199254740992.0`. In financial, identity, or distributed ID systems, this causes silent data corruption.
// improvement: Deserialize large numbers into `serde_json::Number` (preserves exact representation) or a string, then convert explicitly.

use serde::Deserialize;

#[derive(Deserialize)]
struct Data {
    id: f64,
}

fn main() {
    let json = r#"{"id": 9007199254740993}"#;
    let data: Data = serde_json::from_str(json).unwrap();
    println!("{}", data.id as u64);
}
