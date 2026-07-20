// SAFE: OOM handler is defined so `alloc` failure does not panic.
#![no_std]

extern crate alloc;

use alloc::alloc::{alloc, Layout};

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    loop {}
}

pub fn allocate_buffer(size: usize) -> *mut u8 {
    let layout = Layout::from_size_align(size, 4).unwrap();
    unsafe { alloc(layout) }
}
