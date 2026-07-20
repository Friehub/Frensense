// SAFE: Runtime CPU feature check guards the unsafe path even when compiled with cfg.
pub unsafe fn fast_memset(dst: *mut u8, val: u8, count: usize) {
    #[cfg(target_arch = "x86_64")]
    if is_x86_feature_detected!("sse2") {
        return sse2_memset(dst, val, count);
    }
    for i in 0..count {
        *dst.add(i) = val;
    }
}

#[cfg(target_arch = "x86_64")]
unsafe fn sse2_memset(_dst: *mut u8, _val: u8, _count: usize) {}
