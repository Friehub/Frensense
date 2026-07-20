// [frensense]
// observation: Cryptographic key material reinterpreted via std::mem::transmute(), violating type safety guarantees.
// impact: Transmuting key bytes can expose key material through type confusion, cause undefined behavior if layouts mismatch, or silently corrupt key data across features.
// improvement: Use safe conversions like from_ne_bytes(), from_le_bytes(), or dedicated key wrappers instead of transmute.

fn derive_key(seed: &[u8; 32]) -> [u64; 4] {
    // VULNERABLE: transmuting bytes to u64 array
    unsafe { std::mem::transmute::<[u8; 32], [u64; 4]>(*seed) }
}

fn transmute_secret_to_u32(secret: &[u8; 16]) -> [u32; 4] {
    // VULNERABLE: transmuting crypto key to u32 array
    unsafe { std::mem::transmute::<[u8; 16], [u32; 4]>(*secret) }
}
