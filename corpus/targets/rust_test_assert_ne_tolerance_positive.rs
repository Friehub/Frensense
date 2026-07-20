// [frensense]
// observation: `assert_ne!` is used to compare two `f32` or `f64` values without an epsilon tolerance, so values that differ by a tiny floating-point rounding error (e.g., 1e-16) will cause spurious test failures.
// impact: Tests become flaky — they pass on some platforms or compiler optimization levels and fail on others, wasting developer time on false-positive failures in CI.
// improvement: Use `assert!((a - b).abs() >= epsilon)` with a tolerance, or use the `float-cmp` crate for approximate inequality checks.

fn calculate_tax(amount: f64, rate: f64) -> f64 {
    amount * rate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tax_not_equal_to_wrong_value() {
        let tax = calculate_tax(100.0, 0.07);
        assert_ne!(tax, 7.000000000000001);
    }
}
