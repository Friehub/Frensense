// [frensense]
// observation: Multiple fields in a struct use `#[serde(rename = "same")]` pointing to the same serialization name, causing one field to silently overwrite the other during deserialization.
// impact: Data loss — one of the fields will always be missing or wrong after deserialization. Attackers can choose which value wins based on JSON key order.
// improvement: Ensure all `rename` attributes have unique values.

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    #[serde(rename = "endpoint")]
    pub api_url: String,
    #[serde(rename = "endpoint")]
    pub callback_url: String,
}
