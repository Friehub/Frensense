// [frensense]
// observation: str::from_utf8_unchecked() called on user-supplied or untrusted bytes without prior validation.
// impact: Passing invalid UTF-8 to from_utf8_unchecked is immediate undefined behavior. The Rust compiler assumes valid UTF-8 for &str and may eliminate bounds checks or miscompile safety checks.
// improvement: Use str::from_utf8() which returns a Result, or validate with str::from_utf8() before the unchecked version.

fn process_user_input(bytes: &[u8]) -> &str {
    // VULNERABLE: assumes bytes are valid UTF-8
    unsafe { std::str::from_utf8_unchecked(bytes) }
}

fn parse_header_value(raw: &[u8]) -> &str {
    // VULNERABLE: HTTP headers may contain non-UTF-8 bytes
    unsafe { std::str::from_utf8_unchecked(raw) }
}

fn deserialize_name(data: &[u8]) -> &str {
    // VULNERABLE: external data may contain invalid UTF-8
    unsafe { std::str::from_utf8_unchecked(data) }
}
