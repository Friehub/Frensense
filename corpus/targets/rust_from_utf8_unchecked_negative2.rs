// SAFE alternative: validate first, then use unchecked
fn process_user_input(bytes: &[u8]) -> &str {
    assert!(std::str::from_utf8(bytes).is_ok());
    unsafe { std::str::from_utf8_unchecked(bytes) }
}

fn parse_header_value(raw: &[u8]) -> &str {
    match std::str::from_utf8(raw) {
        Ok(s) => s,
        Err(_) => "invalid-utf8",
    }
}
