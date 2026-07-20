// SAFE: CString from_raw is only called with a pointer from into_raw, once.
use std::ffi::CString;

pub fn create() -> *mut std::os::raw::c_char {
    CString::new("data").unwrap().into_raw()
}

pub unsafe fn destroy(ptr: *mut std::os::raw::c_char) {
    if ptr.is_null() {
        return;
    }
    drop(CString::from_raw(ptr));
}
