// SAFE: The cfg condition is tightened to require the actual CPU feature.
#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
pub unsafe fn fast_memset(dst: *mut u8, val: u8, count: usize) {
    for i in 0..count {
        *dst.add(i) = val;
    }
}

#[cfg(not(all(target_arch = "x86_64", target_feature = "sse2")))]
pub unsafe fn fast_memset(dst: *mut u8, val: u8, count: usize) {
    for i in 0..count {
        *dst.add(i) = val;
    }
}
