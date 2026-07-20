// SAFE: Uses ptr::read_unaligned and ptr::write_unaligned to safely access potentially misaligned memory.

use std::ptr;

fn read_u32_aligned(buf: &[u8]) -> u32 {
    let ptr = buf.as_ptr() as *const u32;
    unsafe { ptr::read_unaligned(ptr) }
}

fn write_u64_aligned(buf: &mut [u8]) {
    let ptr = buf.as_mut_ptr() as *mut u64;
    unsafe { ptr::write_unaligned(ptr, 42) };
}
