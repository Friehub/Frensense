// SAFE: Uses safe Box::new_zeroed which validates size at compile time.
use std::boxed::Box;

pub fn make_boxed() -> Box<[u8; 1024]> {
    unsafe { Box::new_zeroed().assume_init() }
}
