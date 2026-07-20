// [frensense]
// observation: A test is annotated with `#[ignore]` but has no reason string, suggesting it was forgotten or the reason for ignoring is undocumented.
// impact: Tests may be silently skipped indefinitely without anyone knowing why, potentially hiding regressions or unfinished work.
// improvement: Always include a reason string in `#[ignore = "reason"]` so the intent is documented.

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
    #[ignore]
    fn test_complex_calculation() {
        assert_eq!(complex_calculation(21), 42);
    }

    #[test]
    #[ignore]
    fn test_external_integration() {
        let result = external_integration().unwrap();
        assert_eq!(result, "expected");
    }
}
