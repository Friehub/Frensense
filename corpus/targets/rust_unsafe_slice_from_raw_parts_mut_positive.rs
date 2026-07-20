// [frensense]
// observation: `std::slice::from_raw_parts_mut` is called with a length that exceeds the actual allocated memory, or a null/unaligned pointer is passed.
// impact: Out-of-bounds read/write causing undefined behavior, memory corruption, or exploitable vulnerabilities.
// improvement: Only call `from_raw_parts_mut` with a pointer and length verified to be within the allocation's bounds.

pub unsafe fn bad_slice(ptr: *mut u8, len: usize) -> &'static mut [u8] {
    std::slice::from_raw_parts_mut(ptr, len)
}
