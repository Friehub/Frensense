use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    role: String,
    debug: Option<String>,
}

fn main() {
    let json = r#"{"role":"user","debug":"true"}"#;
    // SAFE: Explicit fields avoid the flatten override ambiguity entirely.
    let config: Config = serde_json::from_str(json).unwrap();
    println!("role = {}, debug = {:?}", config.role, config.debug);
}
