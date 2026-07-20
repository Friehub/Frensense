// [frensense]
// observation: `CStr::from_ptr` is called with a pointer whose value or provenance comes from untrusted external input (e.g., a raw network buffer) without validation that it points to a valid null-terminated C string.
// impact: An attacker can cause the function to read arbitrary memory (out-of-bounds read) until a null byte is found, leaking sensitive data or causing a page fault denial-of-service.
// improvement: Use `CStr::from_bytes_until_nul` or validate that the pointer is within a known buffer with a size limit.

use std::ffi::CStr;
use std::os::raw::c_char;

pub unsafe fn log_external_message(ptr: *const c_char) {
    let msg = CStr::from_ptr(ptr);
    // msg would read until a null byte — unbounded!
    println!("{:?}", msg);
}
