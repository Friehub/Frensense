// [frensense]
// observation: Casting a larger integer type to a smaller one (e.g., u64 as u32) without checking if the value fits.
// impact: If the value exceeds the target type's max, the cast truncates silently, discarding high bits. A size of 0x1_0000_0001 becomes 1, causing logic errors or security bypasses.
// improvement: Use TryFrom<i from>: for checked conversions, or compare against the target type's MAX before casting.

fn parse_length_from_header(raw: u64) -> u32 {
    // VULNERABLE: truncates values > u32::MAX
    raw as u32
}

fn file_seek_from_user(offset: u64) -> i64 {
    // VULNERABLE: truncates offset > i64::MAX
    offset as i64
}

fn allocate_buffer(size: u64) -> Vec<u8> {
    // VULNERABLE: large size wraps to small u32 capacity
    Vec::with_capacity(size as usize)
}
