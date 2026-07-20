// SAFE: Box is used normally with ownership transfer; no manual drop or raw pointer reuse occurs.

fn safe_use() -> u32 {
    let b = Box::new(42u32);
    *b
}

fn safe_double_drop() {
    let b = Box::new("hello".to_string());
    drop(b);
}
