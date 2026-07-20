// [frensense]
// observation: A function returning `Result` is called and the result is bound to `let _ =`, silently discarding the error without logging or handling.
// impact: Potential failure conditions are ignored, which could lead to data loss, inconsistent state, or silent security bypasses.
// improvement: Log or handle the error instead of silently discarding it, even if using `let _ =`.

use std::fs;

fn cleanup_temp() {
    let _ = fs::remove_file("/tmp/temp_data");
}

fn send_event(event: String) -> Result<(), String> {
    Err(format!("failed to send: {}", event))
}

fn process_events() {
    let _ = send_event("user_login".into());
    let _ = send_event("order_placed".into());
}
