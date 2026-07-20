// SAFE: The FnOnce closure is stored in an Option and taken out before calling, ensuring it's called at most once.

use std::ops::FnOnce;

fn run_once(f: Box<dyn FnOnce()>) {
    let mut maybe = Some(f);
    if let Some(f) = maybe.take() {
        f();
    }
}
