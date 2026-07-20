// SAFE: from_utf8 returns Result, handles invalid input gracefully
fn process_user_input(bytes: &[u8]) -> Result<&str, std::str::Utf8Error> {
    std::str::from_utf8(bytes)
}

fn parse_header_value(raw: &[u8]) -> &str {
    std::str::from_utf8(raw).unwrap_or("invalid-utf8")
}

fn deserialize_name(data: &[u8]) -> String {
    String::from_utf8_lossy(data).to_string()
}
