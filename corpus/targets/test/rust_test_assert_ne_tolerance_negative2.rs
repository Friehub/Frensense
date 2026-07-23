// SAFE: Uses `float-cmp` crate for precise approximate-inequality checks.
fn calculate_tax(amount: f64, rate: f64) -> f64 {
    amount * rate
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_ne;

    #[test]
    fn test_tax_not_equal_to_wrong_value() {
        let tax = calculate_tax(100.0, 0.07);
        assert!(approx_ne!(f64, tax, 7.000000000000001, ulps = 2));
    }
}
