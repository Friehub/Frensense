// SAFE: Uses a logging framework to record errors instead of silently discarding them
use std::fs;
use log::warn;

fn cleanup_temp() {
    if let Err(e) = fs::remove_file("/tmp/temp_data") {
        warn!("cleanup warning: {}", e);
    }
}

fn send_event(event: String) -> Result<(), String> {
    Err(format!("failed to send: {}", event))
}

fn process_events() {
    let results = vec![send_event("user_login".into()), send_event("order_placed".into())];
    for result in results {
        if let Err(e) = result {
            warn!("event error: {}", e);
        }
    }
}
