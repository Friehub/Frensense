// SAFE: Uses safe references instead of raw pointers where possible
fn read_from_ptr(ptr: &i32) -> i32 {
    *ptr
}

fn write_to_ptr(ptr: &mut i32, val: i32) {
    *ptr = val;
}

fn offset_read(slice: &[u8], offset: usize) -> Option<u8> {
    slice.get(offset).copied()
}
