pub fn process(data: &str) -> Result<i32, String> {
    if data.is_empty() {
        return Err("data cannot be empty".to_string());
    }
    Ok(42)
}
