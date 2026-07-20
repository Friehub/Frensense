// SAFE: Uses Arc + Mutex with a Weak reference to break the cycle; also safe in multi-threaded context.

use std::sync::{Arc, Mutex, Weak};

struct SharedState {
    counter: i32,
    on_event: Option<Box<dyn Fn() + Send>>,
}

fn create_cycle_free() -> Arc<Mutex<SharedState>> {
    let state = Arc::new(Mutex::new(SharedState {
        counter: 0,
        on_event: None,
    }));
    let weak = Arc::downgrade(&state);
    let cb = move || {
        if let Some(s) = weak.upgrade() {
            if let Ok(mut guard) = s.lock() {
                guard.counter += 1;
            }
        }
    };
    state.lock().unwrap().on_event = Some(Box::new(cb));
    state
}
