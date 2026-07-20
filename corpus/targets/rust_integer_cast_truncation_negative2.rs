// SAFE alternative: manual bounds check before cast
fn parse_length_from_header(raw: u64) -> Option<u32> {
    if raw > u32::MAX as u64 { None } else { Some(raw as u32) }
}

fn file_seek_from_user(offset: u64) -> Option<i64> {
    if offset > i64::MAX as u64 { None } else { Some(offset as i64) }
}
