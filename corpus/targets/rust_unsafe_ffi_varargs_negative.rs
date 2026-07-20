use std::ffi::CString;

extern "C" {
    fn puts(s: *const std::os::raw::c_char) -> i32;
}

pub unsafe fn safe_print() {
    let msg = CString::new("hello").unwrap();
    puts(msg.as_ptr());
}
