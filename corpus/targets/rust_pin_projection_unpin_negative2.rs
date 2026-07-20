// SAFE: Uses `Box::pin` to heap-allocate and properly pin the `!Unpin` type.
use std::marker::PhantomPinned;
use std::pin::Pin;

pub struct SelfReferential {
    data: String,
    ptr: *const String,
    _pinned: PhantomPinned,
}

impl SelfReferential {
    pub fn new(data: String) -> Pin<Box<Self>> {
        let mut s = Self { ptr: std::ptr::null(), data, _pinned: PhantomPinned };
        s.ptr = &s.data;
        Box::pin(s)
    }
}

pub fn example() -> Pin<Box<SelfReferential>> {
    SelfReferential::new("hello".into())
}
