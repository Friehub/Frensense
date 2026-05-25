fn convert(val: i32) -> u64 {
    unsafe { std::mem::transmute(val) }
}
