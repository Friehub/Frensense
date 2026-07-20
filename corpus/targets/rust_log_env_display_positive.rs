// [frensense]
// observation: A type containing sensitive fields implements Display which includes those fields in its output, and is logged or exposed in error messages.
// impact: Sensitive data such as internal IDs, tokens, or internal state is leaked through the Display formatting.
// improvement: Implement Display to show only non-sensitive fields, or implement a separate debug-like method for internal use.

use std::fmt;

struct ApiKey {
    value: String,
    label: String,
}

impl fmt::Display for ApiKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ApiKey(label={}, value={})", self.label, self.value)
    }
}

fn log_api_key(key: &ApiKey) {
    println!("Using API key: {}", key);
}
