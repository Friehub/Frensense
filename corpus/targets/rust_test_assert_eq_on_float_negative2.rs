// SAFE: Uses the `float-cmp` crate for exact floating-point comparison with ulps
use float_cmp::approx_eq;

fn calculate_discount(price: f64, rate: f64) -> f64 {
    price * (1.0 - rate)
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_discount() {
        let result = calculate_discount(100.0, 0.1);
        assert_approx_eq!(f64, result, 90.0, ulps = 2);
    }

    #[test]
    fn test_tax_calculation() {
        let subtotal: f64 = 10.0;
        let tax_rate: f64 = 0.08;
        let total = subtotal + subtotal * tax_rate;
        assert_approx_eq!(f64, total, 10.8, ulps = 2);
    }
}
