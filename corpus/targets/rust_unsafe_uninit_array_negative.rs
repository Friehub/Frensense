// SAFE: Initializes all elements in the array before reading
use std::mem::MaybeUninit;

fn init_all() -> [i32; 10] {
    let mut arr: [MaybeUninit<i32>; 10] = unsafe { MaybeUninit::uninit().assume_init() };
    for elem in arr.iter_mut() {
        elem.write(0);
    }
    unsafe { std::mem::transmute::<_, [i32; 10]>(arr) }
}
