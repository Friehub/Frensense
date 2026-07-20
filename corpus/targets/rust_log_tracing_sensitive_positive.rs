// [frensense]
// observation: Sensitive data such as authentication tokens, passwords, or API keys are logged via tracing::info! or tracing::debug! macros.
// impact: Sensitive information is written to logs where it may be viewed by unauthorized personnel or captured by log aggregation tools.
// improvement: Redact or omit sensitive fields before logging. Use tracing's sensitive field markers or custom formatting.

use tracing;

fn process_login(username: &str, password: &str) {
    tracing::info!("login attempt: user={}, password={}", username, password);
}

fn handle_token(user: &str, token: &str) {
    tracing::debug!("token for {}: {}", user, token);
}

fn process_payment(card_number: &str) {
    tracing::info!("processing payment with card {}", card_number);
}
