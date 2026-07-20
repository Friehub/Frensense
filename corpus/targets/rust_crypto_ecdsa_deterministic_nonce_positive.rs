// [frensense]
// observation: ECDSA signing uses a deterministic nonce (e.g., RFC 6979) without a backup RNG. If the deterministic nonce generation is predictable or reused across signatures (e.g., due to a fork or VM snapshot), the private key can be recovered.
// impact: Complete private key recovery from two signatures with the same nonce (k-value reuse attack).
// improvement: Use randomized nonce generation or ensure the RNG is seeded with sufficient entropy and never duplicated.

use p256::ecdsa::{SigningKey, signature::Signer};
use rand::rngs::OsRng;

pub fn sign_message(key: &SigningKey, msg: &[u8]) -> Vec<u8> {
    key.sign(msg).to_vec()
}
