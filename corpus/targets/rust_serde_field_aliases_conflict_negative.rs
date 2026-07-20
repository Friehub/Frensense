use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    #[serde(alias = "primary_email")]
    email_address: String,
    #[serde(alias = "backup_email")]
    secondary_email: Option<String>,
}

fn main() {
    let json = r#"{"email": "alice@example.com"}"#;
    // SAFE: Aliases are unique; "email" no longer conflicts.
    // This will fail to deserialize "email", forcing the caller to use the correct keys.
    let user: User = serde_json::from_str(json).unwrap();
    println!("{:?}", user);
}
