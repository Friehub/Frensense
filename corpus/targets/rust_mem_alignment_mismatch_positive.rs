// [frensense]
// observation: A raw pointer is cast from a misaligned byte buffer and dereferenced as a &T without verifying alignment, causing undefined behavior on architectures that require aligned access.
// impact: On ARM, MIPS, and other strict-alignment architectures, misaligned loads cause a SIGBUS crash. On x86 it may cause performance degradation or non-atomic reads of atomic types.
// improvement: Use ptr::read_unaligned or align the buffer before casting; alternatively use byteorder crate or safe deserialization.

use std::mem;

fn read_u32_unaligned(buf: &[u8]) -> u32 {
    let ptr = buf.as_ptr() as *const u32;
    unsafe { *ptr }
}

fn write_u64_unaligned(buf: &mut [u8]) {
    let ptr = buf.as_mut_ptr() as *mut u64;
    unsafe { *ptr = 42 };
}
