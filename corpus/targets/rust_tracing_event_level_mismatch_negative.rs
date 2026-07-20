// SAFE: Sensitive information is logged at DEBUG level only, so it is not included in default production log output.

use tracing::debug;

fn process_user(email: &str, password: &str) {
    debug!("processing user with email: {}", email);
}

fn authenticate(username: &str, token: &str) {
    debug!("authenticating {} with token {}", username, token);
}
