// SAFE: Uses adjacently tagged enum for exact variant matching
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum Command {
    Read { path: String },
    Write { path: String, content: String },
    Delete { path: String },
    Admin { action: String, secret: String },
}

fn execute_command(body: &str) -> Result<(), serde_json::Error> {
    let cmd: Command = serde_json::from_str(body)?;
    Ok(())
}
