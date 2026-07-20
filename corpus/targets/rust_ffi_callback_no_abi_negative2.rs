// SAFE: Uses a type alias to ensure the function pointer type is consistently extern "C".
extern "C" {
    fn register_callback(cb: CallbackFn);
}

type CallbackFn = extern "C" fn(i32) -> i32;

extern "C" fn my_callback(x: i32) -> i32 {
    x * 2
}

pub fn setup() {
    unsafe {
        register_callback(my_callback);
    }
}
