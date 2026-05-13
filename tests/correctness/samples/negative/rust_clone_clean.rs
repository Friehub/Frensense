// tests/correctness/negative/rust_clone_clean.rs
// No Rule Expected
fn main() {
    let v = vec![1, 2, 3];
    let y = v.clone(); // Not in loop
}
