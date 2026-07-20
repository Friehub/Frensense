// [frensense]
// observation: `#[serde(flatten)]` is used on a field of a user-controlled struct, allowing an attacker to inject arbitrary fields into the flattened struct that may override sensitive internal properties.
// impact: Mass assignment via flatten where unexpected fields from user input can set values in the flattened struct (e.g. setting `is_admin`, `role`, or other privileged fields).
// improvement: Avoid `#[serde(flatten)]` on user-deserialized structs, or use `#[serde(deny_unknown_fields)]` at both levels.

use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserProfile {
    pub display_name: String,
    pub bio: String,
    pub avatar_url: String,
}

#[derive(Deserialize)]
pub struct UpdateProfileRequest {
    pub user_id: String,
    #[serde(flatten)]
    pub profile: UserProfile,
}

fn update_profile(body: &str) -> Result<(), serde_json::Error> {
    let req: UpdateProfileRequest = serde_json::from_str(body)?;
    Ok(())
}
