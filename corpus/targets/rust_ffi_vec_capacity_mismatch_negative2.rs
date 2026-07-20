// SAFE: Uses `into_raw_parts` to preserve the exact allocation metadata.
use std::vec::Vec;

pub fn roundtrip(v: Vec<u8>) -> Vec<u8> {
    let (ptr, len, cap) = v.into_raw_parts();
    unsafe { Vec::from_raw_parts(ptr, len, cap) }
}
