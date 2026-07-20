// [frensense]
// observation: A struct uses `#[serde(flatten)]` to capture extra fields into a `HashMap`, but a sibling field shares a name with an expected flatten key. When the JSON contains a key that matches both the sibling field and a flatten entry, serde writes it to the sibling field while the flatten captures other keys — leading to confusion about where data lives.
// impact: If the sibling field is used for validation (e.g., "role") and the flatten map contains a user-controlled override, the attacker's value goes to the flatten map instead of the validated field, bypassing the intended access control structure.
// improvement: Use `#[serde(deny_unknown_fields)]` to avoid ambiguous flatten, or ensure no naming overlap between sibling fields and flatten targets.

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct Config {
    role: String,
    #[serde(flatten)]
    extra: HashMap<String, String>,
}

fn main() {
    let json = r#"{"role":"user","role":"admin","debug":"true"}"#;
    let config: Config = serde_json::from_str(json).unwrap();
    println!("role = {}", config.role);
    println!("extra = {:?}", config.extra);
}
