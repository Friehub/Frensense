// SAFE: Derives the HMAC key from a master secret using HKDF for proper key strength
use hmac::{Hmac, Mac};
use sha2::Sha256;
use hkdf::Hkdf;

fn sign_message(master: &[u8], message: &[u8]) -> Vec<u8> {
    let hk = Hkdf::<Sha256>::new(Some(b"hmac-key"), master);
    let mut key = [0u8; 32];
    hk.expand(b"hmac-signing", &mut key).unwrap();
    let mut mac = Hmac::<Sha256>::new_from_slice(&key).unwrap();
    mac.update(message);
    mac.finalize().into_bytes().to_vec()
}
