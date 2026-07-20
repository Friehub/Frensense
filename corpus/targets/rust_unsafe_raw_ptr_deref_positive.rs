// [frensense]
// observation: A raw pointer is dereferenced inside an unsafe block without ensuring that the pointer is valid, aligned, and non-null.
// impact: Dereferencing an invalid or dangling pointer causes undefined behavior, including segmentation faults or arbitrary memory reads.
// improvement: Always validate that the pointer is non-null, aligned, and points to valid memory before dereferencing.

fn read_from_ptr(ptr: *const i32) -> i32 {
    unsafe { *ptr }
}

fn write_to_ptr(ptr: *mut i32, val: i32) {
    unsafe { *ptr = val; }
}

fn offset_read(base: *const u8, offset: usize) -> u8 {
    unsafe { *base.add(offset) }
}
