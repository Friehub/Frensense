// SAFE: OsRng for cryptographic purposes
use rand::rngs::OsRng;
use rand::RngCore;

fn generate_api_key() -> String {
    let mut rng = OsRng;
    let key: u64 = rng.next_u64();
    format!("key_{:016x}", key)
}

fn generate_nonce() -> [u8; 16] {
    let mut rng = OsRng;
    let mut nonce = [0u8; 16];
    rng.fill_bytes(&mut nonce);
    nonce
}

fn generate_reset_token() -> String {
    let mut rng = OsRng;
    let mut bytes = vec![0u8; 32];
    rng.fill_bytes(&mut bytes);
    hex::encode(bytes)
}
