// [frensense]
// observation: rand::thread_rng() used to generate cryptographic keys, nonces, or tokens.
// impact: thread_rng() is seeded with a 32-bit seed and uses a non-cryptographic PRNG (ChaCha12). It is predictable if an attacker can observe enough outputs or guess the seed.
// improvement: Use rand::rngs::OsRng for all cryptographic key material and security tokens.

use rand::Rng;

fn generate_api_key() -> String {
    let mut rng = rand::thread_rng();
    let key: u64 = rng.gen();
    format!("key_{:016x}", key)
}

fn generate_nonce() -> [u8; 16] {
    let mut rng = rand::thread_rng();
    let mut nonce = [0u8; 16];
    rng.fill(&mut nonce);
    nonce
}

fn generate_reset_token() -> String {
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
    hex::encode(bytes)
}
