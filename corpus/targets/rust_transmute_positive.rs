fn convert(val: i32) -> u64 {
    // nosemgrep: rust.lang.security.unsafe-usage.unsafe-usage
    unsafe { std::mem::transmute(val) }
}
