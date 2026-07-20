// [frensense]
// observation: `rmp_serde::from_slice` or similar MessagePack deserialization is used on data from an untrusted source without a size limit or recursion depth guard.
// impact: Memory exhaustion (OOM) via deep nesting, "billion laughs" style amplification, or malicious data causing panic/UB.
// improvement: Use a streaming decoder with explicit recursion and size limits, or validate the input before deserialization.

use serde::Deserialize;

#[derive(Deserialize)]
pub struct User {
    pub name: String,
}

pub fn parse_user(data: &[u8]) -> Result<User, Box<dyn std::error::Error>> {
    let user: User = rmp_serde::from_slice(data)?;
    Ok(user)
}
