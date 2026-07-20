use p256::ecdsa::{SigningKey, signature::RandomizedSigner};
use rand::rngs::OsRng;

pub fn sign_message(key: &SigningKey, msg: &[u8]) -> Vec<u8> {
    key.sign_with_rng(&mut OsRng, msg).to_vec()
}
