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
    let mut map = std::collections::HashMap::new();
    let len = s.chars().count() as f64;
    if len == 0.0 {
        return 0.0;
    }
    for c in s.chars() {
        *map.entry(c).or_insert(0) += 1;
    }
    let mut entropy = 0.0;
    for count in map.values() {
        let p = (*count as f64) / len;
        entropy -= p * p.log2();
    }
    entropy
}
