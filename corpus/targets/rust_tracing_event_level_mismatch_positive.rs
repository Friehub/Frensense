// [frensense]
// observation: Sensitive data such as user PII, tokens, or internal state is logged at INFO level instead of DEBUG or TRACE, making it visible in production logs by default.
// impact: Personally identifiable information (PII), authentication tokens, or internal secrets are written to production logs, violating compliance (GDPR, HIPAA) and exposing sensitive data to operators.
// improvement: Log sensitive data at DEBUG or TRACE level, or use structured fields that can be redacted by the logging infrastructure.

use tracing::info;

fn process_user(email: &str, password: &str) {
    info!("processing user with email: {}", email);
}

fn authenticate(username: &str, token: &str) {
    info!("authenticating {} with token {}", username, token);
}
