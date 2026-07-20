// [frensense]
// observation: `alloc::alloc::alloc` is called without an OOM handler, causing the embedded device to panic and reset when memory is exhausted.
// impact: In an embedded/no_std context, an allocation failure triggers a panic that cannot be caught, leading to system reset or undefined behavior in interrupt context.
// improvement: Define a custom `#[alloc_error_handler]` that handles OOM gracefully (e.g., reboots with diagnostics), or use a fixed-size allocator that cannot fail.

#![no_std]

extern crate alloc;

use alloc::alloc::{alloc, Layout};

pub fn allocate_buffer(size: usize) -> *mut u8 {
    let layout = Layout::from_size_align(size, 4).unwrap();
    unsafe { alloc(layout) }
}
