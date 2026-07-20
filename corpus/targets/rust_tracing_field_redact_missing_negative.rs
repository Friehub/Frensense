// SAFE: Sensitive fields are explicitly excluded from structured log events; secrets are never passed to tracing macros.

use tracing::info;

fn login(username: &str, password: &str) {
    info!(username, "user login");
}

fn call_api(api_key: &str, endpoint: &str) {
    info!(endpoint, "calling external API");
}
