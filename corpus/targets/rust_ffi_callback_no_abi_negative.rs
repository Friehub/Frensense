// SAFE: The callback is declared as `extern "C" fn` matching the C ABI.
extern "C" {
    fn register_callback(cb: extern "C" fn(i32) -> i32);
}

extern "C" fn my_callback(x: i32) -> i32 {
    x * 2
}

pub fn setup() {
    unsafe {
        register_callback(my_callback);
    }
}
