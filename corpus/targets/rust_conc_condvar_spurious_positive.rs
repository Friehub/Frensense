// [frensense]
// observation: `std::sync::Condvar::wait()` is called without a surrounding predicate loop. The documentation for `Condvar::wait` explicitly states that it may wake spuriously (without the condition being true).
// impact: Spurious wakeups cause the thread to proceed as if the condition is met, leading to logic errors: reading from an empty queue, proceeding before data is ready, or bypassing a synchronization barrier. This can cause data corruption, panics, or security bypasses.
// improvement: Always wrap `Condvar::wait` in a `while` loop that re-checks the predicate after each wakeup.

use std::sync::{Arc, Condvar, Mutex};

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    std::thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut ready = lock.lock().unwrap();
        *ready = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut ready = lock.lock().unwrap();
    cvar.wait(&mut ready).unwrap();
    if *ready {
        println!("ready");
    }
}
