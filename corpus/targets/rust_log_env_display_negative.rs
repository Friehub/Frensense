// SAFE: Display only shows the label, not the sensitive value
use std::fmt;

struct ApiKey {
    value: String,
    label: String,
}

impl fmt::Display for ApiKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ApiKey(label={})", self.label)
    }
}
