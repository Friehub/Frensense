// SAFE: use from_ne_bytes instead of transmute
fn derive_key(seed: &[u8; 32]) -> [u64; 4] {
    let mut out = [0u64; 4];
    for (i, chunk) in seed.chunks_exact(8).enumerate() {
        out[i] = u64::from_ne_bytes(chunk.try_into().unwrap());
    }
    out
}
