// SAFE alternative: ring crate for key generation
use ring::rand::{SecureRandom, SystemRandom};

fn generate_api_key() -> String {
    let rng = SystemRandom::new();
    let mut key = [0u8; 8];
    rng.fill(&mut key).unwrap();
    format!("key_{}", hex::encode(key))
}

fn generate_nonce() -> [u8; 16] {
    let rng = SystemRandom::new();
    let mut nonce = [0u8; 16];
    rng.fill(&mut nonce).unwrap();
    nonce
}
