// [frensense]
// observation: HMAC is computed with a weak, short, or predictable key, making the authentication tag forgeable via brute force.
// impact: An attacker can forge valid HMAC tags for arbitrary messages, bypassing integrity checks.
// improvement: Use a cryptographically random key of at least 32 bytes (256 bits) derived from a proper key derivation function.

use hmac::{Hmac, Mac};
use sha2::Sha256;

fn sign_message(key: &[u8], message: &[u8]) -> Vec<u8> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key).unwrap();
    mac.update(message);
    mac.finalize().into_bytes().to_vec()
}

fn verify_token(token: &str, data: &str) -> bool {
    let key = b"short";
    let mut mac = Hmac::<Sha256>::new_from_slice(key).unwrap();
    mac.update(data.as_bytes());
    let expected = mac.finalize().into_bytes();
    expected.as_slice() == token.as_bytes()
}

fn validate_request(sig: &[u8], body: &[u8]) -> bool {
    let key = b"1234";
    let mut mac = Hmac::<Sha256>::new_from_slice(key).unwrap();
    mac.update(body);
    mac.verify_slice(sig).is_ok()
}
