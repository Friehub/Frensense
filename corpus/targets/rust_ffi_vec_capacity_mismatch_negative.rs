// SAFE: Capacity exactly matches the original allocation.
use std::vec::Vec;

pub unsafe fn rebuild(ptr: *mut u8, len: usize, cap: usize) -> Vec<u8> {
    Vec::from_raw_parts(ptr, len, cap)
}
