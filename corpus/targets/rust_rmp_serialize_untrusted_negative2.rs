// SAFE: Uses a bounded reader to limit input size before deserialization.
use serde::Deserialize;
use std::io::Read;

#[derive(Deserialize)]
pub struct User {
    pub name: String,
}

pub fn parse_user(data: &[u8]) -> Result<User, Box<dyn std::error::Error>> {
    let mut limited = data.take(1024 * 1024);
    let user: User = rmp_serde::from_read(&mut limited)?;
    Ok(user)
}
