// SAFE: Uses a tagged (internally tagged) enum so each variant is uniquely identified by a "type" field
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Command {
    #[serde(rename = "read")]
    Read { path: String },
    #[serde(rename = "write")]
    Write { path: String, content: String },
    #[serde(rename = "delete")]
    Delete { path: String },
    #[serde(rename = "admin")]
    Admin { action: String, secret: String },
}

fn execute_command(body: &str) -> Result<(), serde_json::Error> {
    let cmd: Command = serde_json::from_str(body)?;
    Ok(())
}
