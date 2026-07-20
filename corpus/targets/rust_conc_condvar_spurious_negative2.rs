use std::sync::{Arc, Condvar, Mutex};

fn main() {
    let pair = Arc::new((Mutex::new(0u32), Condvar::new()));
    let pair2 = pair.clone();

    std::thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut counter = lock.lock().unwrap();
        *counter = 42;
        cvar.notify_all();
    });

    let (lock, cvar) = &*pair;
    let mut counter = lock.lock().unwrap();
    // SAFE: Loop re-checks the predicate after every wakeup, preventing spurious wake issues.
    while *counter < 42 {
        counter = cvar.wait(counter).unwrap();
    }
    println!("counter reached 42");
}
