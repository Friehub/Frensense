// [frensense]
// observation: An untagged enum (`#[serde(untagged)]`) has overlapping variants where the first variant always matches, allowing attackers to bypass variant-specific validation logic.
// impact: The wrong variant may be deserialized, bypassing security checks that differ per variant (e.g. a restricted `AdminCommand` vs a permissive `UserCommand`).
// improvement: Use a tagged enum (internal or adjacently tagged) to ensure exact variant matching, or reorder variants to be unambiguous.

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
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
