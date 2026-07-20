// SAFE: Uses `#[serde(deny_unknown_fields)]` on both the outer and flattened struct to block injection
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserProfile {
    pub display_name: String,
    pub bio: String,
    pub avatar_url: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateProfileRequest {
    pub user_id: String,
    #[serde(flatten)]
    pub profile: UserProfile,
}

fn update_profile(body: &str) -> Result<(), serde_json::Error> {
    let req: UpdateProfileRequest = serde_json::from_str(body)?;
    Ok(())
}
