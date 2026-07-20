// SAFE: Tracks initialization count and only reads initialized portion
use std::mem::MaybeUninit;

struct InitBuffer {
    buf: [MaybeUninit<u8>; 1024],
    count: usize,
}

impl InitBuffer {
    fn new() -> Self {
        Self { buf: unsafe { MaybeUninit::uninit().assume_init() }, count: 0 }
    }
    fn push(&mut self, val: u8) {
        if self.count < 1024 {
            self.buf[self.count] = MaybeUninit::new(val);
            self.count += 1;
        }
    }
    fn initialized_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.buf.as_ptr() as *const u8, self.count) }
    }
}
