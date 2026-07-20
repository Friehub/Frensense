// SAFE: Uses counter-based nonce that increments monotonically (safe with single writer)
use ring::aead::{Aes256Gcm, LessSafeKey, Nonce, UnboundKey, NONCE_LEN};
use std::sync::atomic::{AtomicU64, Ordering};

static NONCE_COUNTER: AtomicU64 = AtomicU64::new(1);

fn encrypt_data(key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>, ring::error::Unspecified> {
    let count = NONCE_COUNTER.fetch_add(1, Ordering::SeqCst);
    let mut nonce_bytes = [0u8; NONCE_LEN];
    nonce_bytes[..8].copy_from_slice(&count.to_be_bytes());
    let nonce = Nonce::assume_unique_for_key(nonce_bytes);
    let unbound_key = UnboundKey::new(&Aes256Gcm, key)?;
    let key = LessSafeKey::new(unbound_key);
    let mut in_out = plaintext.to_vec();
    key.seal_in_place_append_tag(nonce, ring::aead::Aad::empty(), &mut in_out)?;
    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&in_out);
    Ok(result)
}
