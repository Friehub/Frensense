// SAFE: Uses a separate DTO for input that only contains allowed fields, then maps to the domain model
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateUserInput {
    pub email: String,
    pub display_name: String,
}

pub struct UpdateUserRequest {
    pub email: String,
    pub display_name: String,
    pub is_admin: bool,
}

fn update_user(body: &str) -> Result<(), serde_json::Error> {
    let input: UpdateUserInput = serde_json::from_str(body)?;
    let request = UpdateUserRequest {
        email: input.email,
        display_name: input.display_name,
        is_admin: false,
    };
    Ok(())
}
