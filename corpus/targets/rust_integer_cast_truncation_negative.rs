// SAFE: TryFrom for checked narrowing casts
use std::convert::TryFrom;

fn parse_length_from_header(raw: u64) -> Result<u32, &'static str> {
    u32::try_from(raw).map_err(|_| "length exceeds u32 range")
}

fn file_seek_from_user(offset: u64) -> Result<i64, &'static str> {
    i64::try_from(offset).map_err(|_| "offset exceeds i64 range")
}

fn allocate_buffer(size: u64) -> Result<Vec<u8>, &'static str> {
    let cap = usize::try_from(size).map_err(|_| "size exceeds usize")?;
    Ok(Vec::with_capacity(cap))
}
