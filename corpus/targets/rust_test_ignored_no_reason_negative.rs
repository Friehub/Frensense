// SAFE: Each `#[ignore]` includes a reason string documenting why the test is disabled
fn complex_calculation(input: i32) -> i32 {
    input * 2
}

fn external_integration() -> Result<String, String> {
    Err("not implemented".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "flaky on CI due to timing; will fix in #123"]
    fn test_complex_calculation() {
        assert_eq!(complex_calculation(21), 42);
    }

    #[test]
    #[ignore = "external API not yet available; tracked in #456"]
    fn test_external_integration() {
        let result = external_integration().unwrap();
        assert_eq!(result, "expected");
    }
}
