// SAFE: Generates a fresh nonce for every encryption using a secure RNG
use ring::aead::{Aes256Gcm, LessSafeKey, Nonce, UnboundKey, NONCE_LEN};
use ring::rand::{SecureRandom, SystemRandom};

fn encrypt_data(key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>, ring::error::Unspecified> {
    let rng = SystemRandom::new();
    let mut nonce_bytes = [0u8; NONCE_LEN];
    rng.fill(&mut nonce_bytes)?;
    let nonce = Nonce::assume_unique_for_key(nonce_bytes);
    let unbound_key = UnboundKey::new(&Aes256Gcm, key)?;
    let key = LessSafeKey::new(unbound_key);
    let mut in_out = plaintext.to_vec();
    key.seal_in_place_append_tag(nonce, ring::aead::Aad::empty(), &mut in_out)?;
    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&in_out);
    Ok(result)
}
