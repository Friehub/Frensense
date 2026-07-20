// SAFE: Out-parameter is zero-initialized before the call, so reads are always defined.
use std::os::raw::c_int;

extern "C" {
    fn get_status(out: *mut c_int) -> c_int;
}

pub fn read_status() -> c_int {
    let mut status: c_int = 0;
    unsafe {
        get_status(&mut status);
    }
    status
}
