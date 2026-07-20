// SAFE: Generates a fresh nonce from OsRng for each encryption
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit, OsRng};

fn encrypt_message(key: &[u8; 32], data: &[u8]) -> Result<Vec<u8>, aes_gcm::Error> {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let mut ct = cipher.encrypt(&nonce, data)?;
    let mut result = nonce.to_vec();
    result.append(&mut ct);
    Ok(result)
}
