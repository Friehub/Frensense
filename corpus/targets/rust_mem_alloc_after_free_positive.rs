// [frensense]
// observation: Memory is manually freed via std::mem::forget combined with a raw pointer drop, then the pointer is re-used, causing a use-after-free.
// impact: Reading or writing freed memory is undefined behavior — may crash, corrupt data, or be exploited for code execution.
// improvement: Use Rust's ownership model instead of manual memory management; avoid mem::forget + manual drop patterns with raw pointers.

use std::mem;
use std::ptr;

fn use_after_free() -> u32 {
    let b = Box::new(42u32);
    let raw = Box::into_raw(b);
    unsafe {
        drop(Box::from_raw(raw));
        let leaked = Box::from_raw(raw);
        *leaked
    }
}

fn double_drop() {
    let b = Box::new("hello".to_string());
    let raw = Box::into_raw(b);
    unsafe {
        mem::forget(Box::from_raw(raw));
        mem::forget(Box::from_raw(raw));
    }
}
