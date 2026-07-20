// [frensense]
// observation: A MaybeUninit array is used (read or passed to a function) before all elements have been initialized, causing undefined behavior.
// impact: Reading uninitialized memory produces undefined behavior, potentially leaking stack contents or causing incorrect program behavior.
// improvement: Ensure every element is initialized before the MaybeUninit array is converted to an initialized type.

use std::mem::MaybeUninit;

fn read_before_init() -> i32 {
    let mut arr: [MaybeUninit<i32>; 10] = MaybeUninit::uninit().assume_init();
    arr[0] = MaybeUninit::new(42);
    unsafe { arr[5].assume_init() }
}

fn partial_init() {
    let mut buf: [MaybeUninit<u8>; 1024] = MaybeUninit::uninit().assume_init();
    for i in 0..100 {
        buf[i] = MaybeUninit::new(i as u8);
    }
    let slice = unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const u8, 1024) };
    process(slice);
}
