// [frensense]
// observation: A custom `Deserialize` implementation for a type returns a value without running any validation on the input, even though the struct invariants require it (e.g., non-empty strings, bounded integers).
// impact: Invalid data enters the system, potentially causing panics, logic bugs, or security vulnerabilities downstream.
// improvement: Perform validation inside the custom deserializer or use `#[serde(deserialize_with)]` that includes checks.

use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess};
use std::fmt;

#[derive(Debug)]
pub struct NonEmptyString(pub String);

impl<'de> Deserialize<'de> for NonEmptyString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(NonEmptyString(s))
    }
}
