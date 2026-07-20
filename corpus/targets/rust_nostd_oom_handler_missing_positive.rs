// [frensense]
// observation: A `#![no_std]` binary uses the `alloc` crate (e.g., `Box`, `Vec`) but does not define an `#[alloc_error_handler]` function.
// impact: If `alloc::alloc::handle_alloc_error` is called (i.e., OOM), the program panics with an unhelpful message because no custom handler is installed. This is especially dangerous on embedded devices where a panic leads to a hard fault or watchdog reset.
// improvement: Define `#[alloc_error_handler]` to handle allocation failures gracefully, e.g., by rebooting or logging diagnostic information.

#![no_std]

extern crate alloc;

use alloc::vec::Vec;

pub fn process_data(len: usize) {
    let mut buf = Vec::<u8>::new();
    buf.resize(len, 0);
}
