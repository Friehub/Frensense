// [frensense]
// observation: `Send` or `Sync` is implemented for a type via `unsafe impl` without any documented justification, invariants, or proof that the type is actually safe to send or share across threads.
// impact: If the type contains a raw pointer, `Cell`, `RefCell`, or other non-thread-safe interior, sending it across threads can cause data races, use-after-free, or other undefined behavior. Future maintainers may add non-Send fields without noticing the unsafe impl.
// improvement: Document the safety invariant in a `// SAFETY:` comment, and audit that all fields are `Send + Sync` or properly guarded.

use std::marker::{Send, Sync};
use std::ptr::NonNull;

pub struct FastBuffer {
    ptr: NonNull<u8>,
    len: usize,
}

unsafe impl Send for FastBuffer {}
unsafe impl Sync for FastBuffer {}

pub fn create_buffer(size: usize) -> FastBuffer {
    let layout = std::alloc::Layout::from_size_align(size, 1).unwrap();
    let ptr = NonNull::new(unsafe { std::alloc::alloc(layout) }).unwrap();
    FastBuffer { ptr, len: size }
}
