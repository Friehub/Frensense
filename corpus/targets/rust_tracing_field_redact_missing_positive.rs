// [frensense]
// observation: A structured log event includes a password, API key, or other secret as a tracing field without redaction, making it visible in log aggregation systems.
// impact: Secrets (passwords, tokens, API keys) are written in plaintext to logs, log files, and log aggregation services (ELK, Datadog, Splunk), leading to credential exposure.
// improvement: Redact or hash sensitive fields before logging, or use a custom wrapper that implements Display/Serialize to mask the value.

use tracing::info;

fn login(username: &str, password: &str) {
    info!(username, password, "user login");
}

fn call_api(api_key: &str, endpoint: &str) {
    info!(api_key, endpoint, "calling external API");
}
