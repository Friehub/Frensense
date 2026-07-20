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
    // SAFE: Predicate loop guards against spurious wakeups — re-check condition after each wake.
    while !*ready {
        ready = cvar.wait(ready).unwrap();
    }
    println!("ready");
}
