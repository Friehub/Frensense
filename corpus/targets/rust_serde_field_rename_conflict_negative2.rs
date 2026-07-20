// SAFE: Unique rename values for each field.
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    #[serde(rename = "api_endpoint")]
    pub api_url: String,
    #[serde(rename = "callback_endpoint")]
    pub callback_url: String,
}
