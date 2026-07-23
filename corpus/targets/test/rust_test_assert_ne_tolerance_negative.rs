// SAFE: Comparison uses epsilon tolerance for floats.
fn calculate_tax(amount: f64, rate: f64) -> f64 {
    amount * rate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tax_not_equal_to_wrong_value() {
        let tax = calculate_tax(100.0, 0.07);
        assert!((tax - 7.000000000000001).abs() > 1e-12, "tax should differ from wrong value");
    }
}
