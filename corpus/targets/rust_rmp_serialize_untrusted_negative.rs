use serde::Deserialize;

#[derive(Deserialize)]
pub struct User {
    pub name: String,
}

pub fn parse_user(data: &[u8]) -> Result<User, Box<dyn std::error::Error>> {
    if data.len() > 1024 * 1024 {
        return Err("input too large".into());
    }
    let user: User = rmp_serde::from_slice(data)?;
    Ok(user)
}
