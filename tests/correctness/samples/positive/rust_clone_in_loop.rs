// tests/correctness/positive/rust_clone_in_loop.rs
// Rule: RUST_CLONE_IN_LOOP
fn main() {
    let v = vec![1, 2, 3];
    for x in v {
        let y = x.clone();
    }
}
