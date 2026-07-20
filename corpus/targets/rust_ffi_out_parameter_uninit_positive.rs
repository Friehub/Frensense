// [frensense]
// observation: A C function is called via FFI with an out-parameter (e.g., `*mut T`), but the Rust code reads the value from that pointer _before_ the C function writes to it, meaning it reads uninitialized memory.
// impact: Reading uninitialized memory is undefined behavior in Rust. The value read is garbage and can lead to arbitrary logic errors, and in optimized builds the compiler may assume the value is valid, leading to UB.
// improvement: Initialize the out-parameter to a known value (e.g., zeroed) before passing the pointer, or use `MaybeUninit` and only read after the call.

use std::os::raw::c_int;

extern "C" {
    fn get_status(out: *mut c_int) -> c_int;
}

pub fn read_status() -> c_int {
    let mut status: c_int = unsafe { std::mem::uninitialized() };
    unsafe {
        get_status(&mut status);
    }
    status
}
