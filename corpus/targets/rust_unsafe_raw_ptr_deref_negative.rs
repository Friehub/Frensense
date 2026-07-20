// SAFE: Checks pointer validity before dereferencing
fn read_from_ptr(ptr: *const i32) -> Option<i32> {
    if ptr.is_null() { return None; }
    if (ptr as usize) % std::mem::align_of::<i32>() != 0 { return None; }
    Some(unsafe { *ptr })
}

fn write_to_ptr(ptr: *mut i32, val: i32) {
    if ptr.is_null() { return; }
    unsafe { *ptr = val; }
}
