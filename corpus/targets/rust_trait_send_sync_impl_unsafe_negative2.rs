// SAFE: Wraps fields to avoid unsafe Send/Sync impl entirely.
use std::marker::Send;
use std::ptr::NonNull;

pub struct FastBuffer {
    ptr: NonNull<u8>,
    len: usize,
    _not_send: PhantomData<*mut u8>,
}

impl FastBuffer {
    pub fn new(ptr: NonNull<u8>, len: usize) -> Self {
        Self { ptr, len, _not_send: PhantomData }
    }
}
