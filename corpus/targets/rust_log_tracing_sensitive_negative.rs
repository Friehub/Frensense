// SAFE: Sensitive fields are redacted before logging
use tracing;

fn process_login(username: &str, _password: &str) {
    tracing::info!("login attempt: user={}", username);
}

fn handle_token(user: &str, _token: &str) {
    tracing::debug!("token processed for {}", user);
}

fn process_payment(card_number: &str) {
    let last4 = &card_number[card_number.len().saturating_sub(4)..];
    tracing::info!("processing payment with card ending in {}", last4);
}
