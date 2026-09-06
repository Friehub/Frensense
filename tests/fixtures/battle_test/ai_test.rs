fn ai_test_scenarios(x: i32) {
    // 1. Tautological Assert (Fix 3 verification)
    // Frensense v0.2.0 only caught assert!(true).
    // Frensense v0.3.0 should catch this via AST comparison.
    assert!(x == x);

    // 2. Placeholder Panic
    // Should catch this unconditionally now.
    todo!("Implement this");
}
