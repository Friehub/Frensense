// SAFE: Uses counter-based nonce with atomic increment (safe in single-process scenarios)
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(1);

fn encrypt_message(key: &[u8; 32], data: &[u8]) -> Result<Vec<u8>, aes_gcm::Error> {
    let count = COUNTER.fetch_add(1, Ordering::SeqCst);
    let mut nonce_bytes = [0u8; 12];
    nonce_bytes[4..].copy_from_slice(&count.to_be_bytes());
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    let mut ct = cipher.encrypt(Nonce::from_slice(&nonce_bytes), data)?;
    let mut result = nonce_bytes.to_vec();
    result.append(&mut ct);
    Ok(result)
}
