// SAFE: Uses an epsilon comparison for floating-point assertions
fn calculate_discount(price: f64, rate: f64) -> f64 {
    price * (1.0 - rate)
}

fn approx_eq(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discount() {
        let result = calculate_discount(100.0, 0.1);
        assert!(approx_eq(result, 90.0, 1e-10));
    }

    #[test]
    fn test_tax_calculation() {
        let subtotal: f64 = 10.0;
        let tax_rate: f64 = 0.08;
        let total = subtotal + subtotal * tax_rate;
        assert!((total - 10.8).abs() < 1e-10);
    }
}
