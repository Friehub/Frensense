// SAFE: Uses `#[serde(deny_unknown_fields)]` to reject unexpected fields
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateUserRequest {
    pub email: String,
    pub display_name: String,
}

fn update_user(body: &str) -> Result<(), serde_json::Error> {
    let req: UpdateUserRequest = serde_json::from_str(body)?;
    Ok(())
}
