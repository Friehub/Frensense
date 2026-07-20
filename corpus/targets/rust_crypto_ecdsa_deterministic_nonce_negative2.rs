// SAFE: Uses randomized signing with a fresh thread-local RNG.
use p256::ecdsa::{SigningKey, signature::RandomizedSigner};

pub fn sign_message(key: &SigningKey, msg: &[u8]) -> Vec<u8> {
    key.sign_with_rng(&mut rand::thread_rng(), msg).to_vec()
}
