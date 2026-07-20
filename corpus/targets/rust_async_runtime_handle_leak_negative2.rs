// SAFE: Spawns a dedicated std::thread for blocking work instead of nesting runtimes
use std::thread;

async fn handler() -> String {
    let result = thread::spawn(|| compute_heavy())
        .join()
        .unwrap();
    format!("result: {}", result)
}
