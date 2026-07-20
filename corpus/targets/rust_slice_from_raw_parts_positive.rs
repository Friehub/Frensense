// [frensense]
// observation: std::slice::from_raw_parts() called with a length derived from user input or external data without validation.
// impact: Passing an attacker-controlled length to from_raw_parts can cause out-of-bounds reads, memory disclosure, or segmentation faults. The function is unsafe because Rust cannot verify the pointer validity.
// improvement: Validate the length against the actual allocation size before calling from_raw_parts, or use safe abstractions like Vec::from_raw_parts.

fn parse_custom_type(ptr: *const u8, count: usize) -> &'static [u8] {
    // VULNERABLE: attacker-controlled count could read past buffer
    unsafe { std::slice::from_raw_parts(ptr, count) }
}

fn read_packet_data(ptr: *const u8, header_size: usize) -> &'static [u8] {
    // VULNERABLE: header_size from network data
    unsafe { std::slice::from_raw_parts(ptr, header_size) }
}
