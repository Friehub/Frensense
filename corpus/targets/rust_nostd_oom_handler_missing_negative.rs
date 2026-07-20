// SAFE: An `#[alloc_error_handler]` is defined, so OOM is handled without a panic.
#![no_std]

extern crate alloc;

use alloc::alloc::Layout;
use alloc::vec::Vec;

#[alloc_error_handler]
fn oom(_layout: Layout) -> ! {
    loop {}
}

pub fn process_data(len: usize) {
    let mut buf = Vec::<u8>::new();
    buf.resize(len, 0);
}
