// SAFE: The byte slice is bounded and validated before creating a CStr.
use std::ffi::CStr;

pub fn log_external_message(buf: &[u8]) {
    if let Some(end) = buf.iter().position(|&b| b == 0) {
        let slice = &buf[..end];
        if let Ok(s) = core::str::from_utf8(slice) {
            println!("{}", s);
        }
    }
}
