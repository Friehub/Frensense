// SPDX-License-Identifier: MIT

pub const MEDIUM_ENTROPY_THRESHOLD: f64 = 4.0;
pub const HIGH_ENTROPY_THRESHOLD: f64 = 5.5;
pub const MIN_LENGTH_FOR_ENTROPY: usize = 16;
pub const UNCONDITIONAL_ENTROPY_LENGTH: usize = 32;

pub fn is_secret_indicator(name: &str) -> bool {
    let lower = name.to_lowercase();
    let indicators = [
        "key",
        "secret",
        "token",
        "password",
        "passwd",
        "pwd",
        "apikey",
        "api_key",
        "auth",
        "credential",
        "cred",
        "cert",
        "private",
        "priv",
        "signing",
        "encryption",
        "webhook",
        "hmac",
        "bearer",
        "oauth",
        "jwt",
        "salt",
        "seed",
        "nonce",
        "passphrase",
        "pin",
        "otp",
    ];
    indicators.iter().any(|&ind| lower.contains(ind))
}

pub fn calculate_shannon_entropy(s: &str) -> f64 {
    let len = s.len();
    if len == 0 {
        return 0.0;
    }
    let mut counts = [0u32; 128];
    for &b in s.as_bytes() {
        counts[b as usize] += 1;
    }
    let len_f = len as f64;
    counts
        .iter()
        .filter(|&&c| c > 0)
        .map(|&c| {
            let p = c as f64 / len_f;
            -p * p.log2()
        })
        .sum()
}
