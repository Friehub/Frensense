// SAFE alternative: manual enum deserialization with allowlist
use serde::Deserialize;

#[derive(Debug)]
enum SafeAction {
    Read(String),
    Write(String, String),
    Delete(String),
}

impl<'de> Deserialize<'de> for SafeAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        match s.as_str() {
            "read" | "write" | "delete" => {}
            _ => return Err(serde::de::Error::unknown_variant(&s, &["read", "write", "delete"])),
        }
        unimplemented!()
    }
}
