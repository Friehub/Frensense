/// SAFE: `ptr` must be non-null and valid for reads for `len` bytes. `len` must not exceed the allocation.
unsafe fn get_unchecked_ref<T>(ptr: *const T) -> &'static T {
    // SAFE: Caller guarantees pointer is non-null, aligned, and points to a valid initialized value.
    &*ptr
}

fn main() {
    let x = 42u8;
    let r = unsafe { get_unchecked_ref(&x) };
    println!("{}", r);
}
