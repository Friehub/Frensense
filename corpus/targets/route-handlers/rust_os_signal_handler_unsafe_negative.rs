// SAFE: Signal handler only sets an atomic flag; actual work is deferred to the main loop, avoiding non-signal-safe operations.

use std::sync::atomic::{AtomicBool, Ordering};

static TERMINATE: AtomicBool = AtomicBool::new(false);

fn install_safe_handler() {
    signal_hook::flag::register(signal_hook::consts::SIGTERM, || {
        TERMINATE.store(true, Ordering::Relaxed);
    })
    .expect("register failed");
}

fn worker_loop() {
    while !TERMINATE.load(Ordering::Relaxed) {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
