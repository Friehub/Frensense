// SAFE: restrict deserialization to allowed variants
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    action: String,
    path: String,
    data: Option<String>,
}

fn handle_request(body: &[u8]) {
    let config: Config = serde_json::from_slice(body).unwrap();
    match config.action.as_str() {
        "read" => read_file(&config.path),
        "write" => {
            if let Some(data) = config.data {
                write_file(&config.path, &data);
            }
        }
        "delete" => delete_file(&config.path),
        _ => println!("Unknown action: {}", config.action),
    }
}
