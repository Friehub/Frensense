// [frensense]
// observation: A callback function pointer is passed to an `extern "C"` FFI function without specifying the ABI for the callback itself, so the callback may use the wrong calling convention (Rust ABI vs C ABI), causing stack corruption.
// impact: When the C library calls back into Rust with the wrong calling convention, registers and stack can be corrupted, leading to undefined behavior, crashes, or exploitable memory corruption.
// improvement: Declare the callback as `extern "C" fn(...)` to match the C ABI.

extern "C" {
    fn register_callback(cb: fn(i32) -> i32);
}

fn my_callback(x: i32) -> i32 {
    x * 2
}

pub fn setup() {
    unsafe {
        register_callback(my_callback);
    }
}
