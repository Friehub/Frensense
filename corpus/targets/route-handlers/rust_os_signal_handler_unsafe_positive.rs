// [frensense]
// observation: A signal handler registered via signal_hook or libc::signal performs non-signal-safe operations such as memory allocation, locking, or writing to shared state.
// impact: Undefined behavior: signal handlers can interrupt the program at any point; calling non-reentrant functions (malloc, locking, std::io) causes deadlocks, data corruption, or crashes.
// improvement: Only use signal-safe functions (write, sig_atomic_t) inside signal handlers; defer complex work to a dedicated signal-handling thread via signalfd or pipe.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

fn install_handler() {
    let lock = Mutex::new(0u32);
    signal_hook::flag::register(signal_hook::consts::SIGTERM, || {
        let mut data = lock.lock().unwrap();
        *data += 1;
        println!("handled signal"); // malloc inside handler
    })
    .expect("register failed");
}

fn setup_crash_handler() {
    signal_hook::flag::register(signal_hook::consts::SIGSEGV, || {
        let mut v = Vec::with_capacity(1024); // alloc inside handler
        v.push(1u8);
    })
    .expect("register failed");
}
