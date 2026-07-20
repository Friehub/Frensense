// SAFE: Avoids flatten entirely by nesting the profile under a separate key
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
    pub profile: UserProfile,
}

fn update_profile(body: &str) -> Result<(), serde_json::Error> {
    let req: UpdateProfileRequest = serde_json::from_str(body)?;
    Ok(())
}
