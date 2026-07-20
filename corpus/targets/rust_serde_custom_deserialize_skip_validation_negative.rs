use serde::de::{self, Deserialize, Deserializer, Visitor};
use std::fmt;

#[derive(Debug)]
pub struct NonEmptyString(pub String);

impl<'de> Deserialize<'de> for NonEmptyString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            return Err(de::Error::custom("string must not be empty"));
        }
        Ok(NonEmptyString(s))
    }
}
