// [frensense]
// observation: A `#[cfg()]` attribute is used to conditionally enable a block of `unsafe` code (e.g., inline assembly, raw pointer manipulation) that assumes a specific platform or CPU feature, but the condition is too broad or wrong, so the unsafe code runs on an incompatible target.
// impact: The unsafe code can execute undefined behavior on targets that do not have the expected CPU feature or memory layout, causing crashes, data corruption, or exploitable vulnerabilities across platforms.
// improvement: Constrain the `#[cfg()]` predicate to exactly match the required target (e.g., `target_arch`, `target_feature`), and add a runtime check as a safety net.

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
pub unsafe fn fast_memset(dst: *mut u8, val: u8, count: usize) {
    for i in 0..count {
        *dst.add(i) = val;
    }
}
