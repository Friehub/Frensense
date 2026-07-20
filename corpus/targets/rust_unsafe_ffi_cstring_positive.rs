// [frensense]
// observation: `CString::from_raw()` is called on a pointer that was not allocated by `CString::into_raw()`, or the same pointer is freed twice via `from_raw`.
// impact: Double-free, use-after-free, or undefined behavior. An attacker controlling the memory layout could achieve arbitrary code execution.
// improvement: Only pair `from_raw()` with a prior `into_raw()` from the same `CString`, and ensure it is called exactly once.

use std::ffi::CString;

pub unsafe fn bad_from_raw(ptr: *mut std::os::raw::c_char) {
    let _ = CString::from_raw(ptr);
}
