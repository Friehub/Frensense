use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub api_endpoint: String,
    pub callback_endpoint: String,
}
