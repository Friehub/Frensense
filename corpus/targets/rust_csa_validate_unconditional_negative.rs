// Rule: RUST_CSA_VALIDATE_UNCONDITIONAL (negative — no rule expected)
fn validate_input(input: i32) -> bool {
    if input > 0 {
        return false;
    }
    true
}
