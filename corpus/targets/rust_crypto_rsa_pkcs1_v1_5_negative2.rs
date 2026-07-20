// SAFE: Uses RSA-PSS which is the modern, IND-CCA2 secure signature scheme.
use rsa::pss::BlindedSigningKey;
use rsa::signature::RandomizedSigner;
use rsa::RsaPrivateKey;

pub fn sign_data_pss(key: &RsaPrivateKey, data: &[u8]) -> Vec<u8> {
    let signing_key = BlindedSigningKey::new(key.clone());
    signing_key.sign_with_rng(&mut rand::thread_rng(), data).to_vec()
}
