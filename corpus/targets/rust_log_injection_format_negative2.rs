// SAFE: Used structured key-value logging with the `slog` crate, keeping user input in a separate field from the log message.

use slog::{o, info, Drain};
use std::sync::Arc;

fn init_logger() -> slog::Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    slog::Logger::root(drain, o!())
}

fn handle_login(log: &slog::Logger, username: &str, ip: &str) {
    info!(log, "User login";
        "username" => username,
        "ip" => ip,
    );
}

fn process_feedback(log: &slog::Logger, user_input: &str) {
    info!(log, "User feedback received";
        "feedback" => user_input,
    );
}
