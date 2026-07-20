// SAFE: Uses deserialize_with that validates before construction.
use serde::{Deserialize, Deserializer};

fn non_empty<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        return Err(serde::de::Error::custom("must not be empty"));
    }
    Ok(s)
}

#[derive(Deserialize)]
pub struct FormData {
    #[serde(deserialize_with = "non_empty")]
    pub name: String,
}
