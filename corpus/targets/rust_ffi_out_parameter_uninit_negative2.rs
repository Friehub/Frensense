// SAFE: Uses `MaybeUninit` to correctly handle the uninitialized out-parameter.
use std::mem::MaybeUninit;
use std::os::raw::c_int;

extern "C" {
    fn get_status(out: *mut c_int) -> c_int;
}

pub fn read_status() -> c_int {
    let mut status = MaybeUninit::<c_int>::uninit();
    let ret = unsafe { get_status(status.as_mut_ptr()) };
    if ret == 0 {
        unsafe { status.assume_init() }
    } else {
        -1
    }
}
