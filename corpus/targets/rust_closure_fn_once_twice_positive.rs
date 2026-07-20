// [frensense]
// observation: A Box<dyn FnOnce> closure is stored and called multiple times by taking ownership via Box::into_raw or unsafe pointer manipulation, violating the FnOnce contract.
// impact: Calling a FnOnce closure twice is undefined behavior — the closure's captured values may have been moved/dropped, causing use-after-free or double-free.
// improvement: Use FnMut or store the closure behind an Option and use .take() to ensure it's called at most once.

use std::ops::FnOnce;

fn run_twice(f: Box<dyn FnOnce()>) {
    let raw = Box::into_raw(f);
    let f1 = unsafe { Box::from_raw(raw) };
    let f2 = unsafe { Box::from_raw(raw) };
    f1();
    f2();
}

fn call_with_mut(f: &mut dyn FnOnce(&mut String)) -> String {
    let mut s = String::new();
    f(&mut s);
    f(&mut s);
    s
}
