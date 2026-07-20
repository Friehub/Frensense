// SAFE: Sensitive fields are not logged at all; only non-sensitive metadata is emitted at INFO level.

use tracing::info;

fn process_user(email: &str, password: &str) {
    info!("processing user");
}

fn authenticate(username: &str, token: &str) {
    info!("authentication attempt");
}
