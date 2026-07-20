// SAFE: Removes `#[ignore]` on tests that can now run, or replaces with `#[should_panic]` for known failures
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
    fn test_complex_calculation() {
        assert_eq!(complex_calculation(21), 42);
    }

    #[test]
    #[should_panic(expected = "not implemented")]
    fn test_external_integration() {
        let result = external_integration().unwrap();
        assert_eq!(result, "expected");
    }
}
