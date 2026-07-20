// [frensense]
// observation: `assert_eq!` or `assert_ne!` is used to compare `f32` or `f64` values, which is unreliable due to floating-point rounding errors.
// impact: Tests may fail spuriously due to minor floating-point precision differences (e.g. 0.1 + 0.2 != 0.3), or worse, pass when values are meaningfully different due to NaN semantics.
// improvement: Use `assert!((a - b).abs() < epsilon)` with an appropriate tolerance, or use the `float-cmp` crate.

fn calculate_discount(price: f64, rate: f64) -> f64 {
    price * (1.0 - rate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discount() {
        let result = calculate_discount(100.0, 0.1);
        assert_eq!(result, 90.0);
    }

    #[test]
    fn test_tax_calculation() {
        let subtotal: f64 = 10.0;
        let tax_rate: f64 = 0.08;
        let total = subtotal + subtotal * tax_rate;
        assert_eq!(total, 10.8);
    }
}
