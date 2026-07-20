// SAFE: Sensitive fields are redacted via a wrapper type that implements tracing::Value with masked output.

use tracing::info;

struct Redacted(String);

impl tracing::Value for Redacted {
    fn record(&self, key: &tracing::field::Visit) -> Result<(), tracing::field::Error> {
        key.record_str("REDACTED")
    }
}

fn login(username: &str, password: &str) {
    info!(username, password = %Redacted(password.to_string()), "user login");
}
