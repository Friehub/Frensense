// SAFE: Uses a cryptographically strong key of sufficient length
use hmac::{Hmac, Mac};
use sha2::Sha256;

fn verify_token(token: &str, data: &str) -> bool {
    let key = b"this-is-a-256-bit-key-xxxxxxxxxxxxxxxxxxxx";
    let mut mac = Hmac::<Sha256>::new_from_slice(key).unwrap();
    mac.update(data.as_bytes());
    mac.verify_slice(token.as_bytes()).is_ok()
}
