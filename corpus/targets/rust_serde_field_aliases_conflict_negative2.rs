use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    #[serde(rename = "email")]
    email_address: String,
    secondary_email: Option<String>,
}

fn main() {
    let json = r#"{"email": "alice@example.com"}"#;
    // SAFE: Using `rename` instead of `alias` makes "email" the canonical name;
    // only `email_address` matches, avoiding ambiguity.
    let user: User = serde_json::from_str(json).unwrap();
    println!("email: {:?}, secondary: {:?}", user.email_address, user.secondary_email);
}
