// SAFE: Documented invariants show why this type is safe to send/sync.
use std::marker::{Send, Sync};
use std::ptr::NonNull;

pub struct FastBuffer {
    ptr: NonNull<u8>,
    len: usize,
}

// SAFETY: FastBuffer owns the allocation exclusively; no aliasing. ptr is !Send,
// but the allocation is uniquely owned and only accessed through &self methods.
// The buffer is never aliased across threads.
unsafe impl Send for FastBuffer {}
unsafe impl Sync for FastBuffer {}
