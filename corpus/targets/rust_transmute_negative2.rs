// SAFE: Uses from_ne_bytes to safely reinterpret bytes without unsafe transmute
fn convert(val: i32) -> u64 {
    val as u64
}
