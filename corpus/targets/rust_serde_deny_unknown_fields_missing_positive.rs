// [frensense]
// observation: A struct used for deserialization does not have `#[serde(deny_unknown_fields)]`, allowing an attacker to inject unexpected fields that may be processed by other code or stored in a database.
// impact: Mass assignment vulnerability where extra fields in the JSON payload can set internal fields (e.g. `is_admin: true`) if those fields exist, or bypass security checks through unexpected field injection in flattened structs.
// improvement: Add `#[serde(deny_unknown_fields)]` to all deserialization structs to reject unknown fields.

use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub email: String,
    pub display_name: String,
    pub is_admin: bool,
}

fn update_user(body: &str) -> Result<(), serde_json::Error> {
    let req: UpdateUserRequest = serde_json::from_str(body)?;
    Ok(())
}
