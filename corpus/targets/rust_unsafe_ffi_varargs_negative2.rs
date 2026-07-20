// SAFE: Uses fixed-argument C function instead of varargs.
use std::ffi::CString;

extern "C" {
    fn write(fd: i32, buf: *const std::os::raw::c_void, count: usize) -> isize;
}

pub unsafe fn safe_write(msg: &str) {
    let c_msg = CString::new(msg).unwrap();
    write(1, c_msg.as_ptr() as *const std::os::raw::c_void, msg.len());
}
