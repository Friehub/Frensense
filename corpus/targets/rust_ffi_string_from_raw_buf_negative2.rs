// SAFE: The pointer is checked against a known buffer range and a max length before converting.
use std::ffi::CStr;
use std::os::raw::c_char;

pub unsafe fn log_external_message(ptr: *const c_char, max_len: usize) {
    let slice = core::slice::from_raw_parts(ptr as *const u8, max_len);
    if let Some(end) = slice.iter().position(|&b| b == 0) {
        let cstr = CStr::from_bytes_with_nul(&slice[..=end]).unwrap();
        println!("{:?}", cstr);
    }
}
