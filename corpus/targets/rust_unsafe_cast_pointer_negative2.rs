// SAFE: Uses safe conversion methods when types have known compatible layouts
fn cast_u32_to_f32(val: u32) -> f32 {
    f32::from_bits(val)
}

fn cast_bytes_to_u32s(data: &[u8]) -> Option<Vec<u32>> {
    if data.len() % 4 != 0 { return None; }
    Some(data.chunks_exact(4).map(|c| u32::from_ne_bytes([c[0], c[1], c[2], c[3]])).collect())
}
