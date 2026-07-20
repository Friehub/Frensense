// [frensense]
// observation: CTR mode encryption reuses the same nonce/IV across multiple messages, enabling keystream recovery by an attacker.
// impact: An attacker who collects two ciphertexts encrypted with the same CTR nonce can XOR them to recover the plaintext.
// improvement: Generate a unique nonce for every CTR mode encryption, never reuse a (key, nonce) pair.

use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit, OsRng};

fn encrypt_messages(key: &[u8; 32], messages: &[&[u8]]) -> Result<Vec<Vec<u8>>, aes_gcm::Error> {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    let fixed_nonce = Nonce::from_slice(b"unique nonce"); // actually reused!
    let mut results = Vec::new();
    for msg in messages {
        let ct = cipher.encrypt(fixed_nonce, msg)?;
        results.push(ct);
    }
    Ok(results)
}

fn insecure_ctr_encrypt(data: &[u8], key: &[u8]) -> Result<Vec<u8>, aes_gcm::Error> {
    let mut nonce = [0u8; 12];
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    cipher.encrypt(Nonce::from_slice(&nonce), data)
}
