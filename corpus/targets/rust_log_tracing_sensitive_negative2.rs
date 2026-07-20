// SAFE: Uses tracing's sensitive span field annotation to prevent accidental emission
use tracing;

fn process_login(username: &str, password: &str) {
    let span = tracing::span!(tracing::Level::INFO, "login", username = %username, password = tracing::field::Empty);
    let _guard = span.enter();
    tracing::event!(tracing::Level::INFO, "login processed");
}

fn handle_token(user: &str, _token: &str) {
    tracing::info!(user = %user, "token event");
}
