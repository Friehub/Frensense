// [frensense]
// observation: Two struct fields are annotated with `#[serde(alias = "...")]` using the same alias string. When the alias appears in the input, serde can deserialize into either field depending on field order, leading to data ending up in the wrong field.
// impact: Data mapped to the wrong field causes silent corruption. For example, an alias `"email"` on both `email_address` and `secondary_email` means input `email:` could populate either field — the application then processes the wrong value.
// improvement: Ensure aliases are unique across all fields, or avoid aliases in favor of `#[serde(rename = "...")]`.

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    #[serde(alias = "email")]
    email_address: String,
    #[serde(alias = "email")]
    secondary_email: Option<String>,
}

fn main() {
    let json = r#"{"email": "alice@example.com"}"#;
    let user: User = serde_json::from_str(json).unwrap();
    println!("{:?}", user);
}
