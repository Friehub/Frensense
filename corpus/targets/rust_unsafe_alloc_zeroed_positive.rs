// [frensense]
// observation: `alloc::alloc_zeroed` is called without checking the `Layout` size against the deallocation alignment or capacity, which can lead to buffer over-reads if the caller assumes a different size.
// impact: Buffer overflow, out-of-bounds read/write, or undefined behavior if the returned pointer is used beyond the allocated region.
// improvement: Always check that the layout size matches expectations and use safe abstractions like `Vec::with_capacity` or `Box::new_zeroed`.

use std::alloc::{alloc_zeroed, Layout};

pub unsafe fn make_buffer(size: usize) -> *mut u8 {
    let layout = Layout::from_size_align(size, 1).unwrap();
    alloc_zeroed(layout)
}
