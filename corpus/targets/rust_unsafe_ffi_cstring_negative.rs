use std::ffi::CString;

pub fn safe_cstring() -> *mut std::os::raw::c_char {
    CString::new("hello").unwrap().into_raw()
}

pub unsafe fn reclaim_cstring(ptr: *mut std::os::raw::c_char) {
    let _ = CString::from_raw(ptr);
}
