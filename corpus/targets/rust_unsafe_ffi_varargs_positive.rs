// [frensense]
// observation: Calling a C varargs function (e.g., `printf`) with the wrong argument types or count via FFI — the compiler cannot check the variadic arguments.
// impact: Undefined behavior — stack corruption, wrong output, or arbitrary code execution depending on the ABI mismatch.
// improvement: Avoid variadic FFI calls; use safe Rust alternatives or wrap with fixed-argument functions.

extern "C" {
    fn printf(fmt: *const std::os::raw::c_char, ...) -> i32;
}

pub unsafe fn bad_printf() {
    let msg = std::ffi::CString::new("hello %d %s").unwrap();
    printf(msg.as_ptr(), 42, 3.14);
}
