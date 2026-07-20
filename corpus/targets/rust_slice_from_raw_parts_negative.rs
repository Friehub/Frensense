// SAFE: bound-check the length against known allocation size
fn parse_custom_type(ptr: *const u8, count: usize, actual_len: usize) -> Option<&'static [u8]> {
    if count > actual_len {
        return None;
    }
    Some(unsafe { std::slice::from_raw_parts(ptr, count) })
}

fn read_packet_data(buf: &[u8], start: usize, count: usize) -> Option<&[u8]> {
    buf.get(start..start + count)
}
