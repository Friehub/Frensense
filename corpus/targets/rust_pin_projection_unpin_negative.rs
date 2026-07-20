// SAFE: The struct is `!Unpin` via `PhantomPinned`, so `Pin::new` actually pins.
use std::marker::PhantomPinned;
use std::pin::Pin;

pub struct SelfReferential {
    data: String,
    ptr: *const String,
    _pinned: PhantomPinned,
}

impl SelfReferential {
    pub fn new(data: String) -> Self {
        Self { ptr: std::ptr::null(), data, _pinned: PhantomPinned }
    }

    pub fn init(self: Pin<&mut Self>) {
        let this = unsafe { self.get_unchecked_mut() };
        this.ptr = &this.data;
    }
}

pub fn example() {
    let mut val = SelfReferential::new("hello".into());
    let pinned = Pin::new(&mut val);
    // Now Pin actually prevents moves!
}
