// SAFE: Errors are logged before being discarded, so failures are visible in monitoring
use std::fs;

fn cleanup_temp() {
    if let Err(e) = fs::remove_file("/tmp/temp_data") {
        eprintln!("cleanup warning: {}", e);
    }
}

fn send_event(event: String) -> Result<(), String> {
    Err(format!("failed to send: {}", event))
}

fn process_events() {
    if let Err(e) = send_event("user_login".into()) {
        eprintln!("event error: {}", e);
    }
    if let Err(e) = send_event("order_placed".into()) {
        eprintln!("event error: {}", e);
    }
}
