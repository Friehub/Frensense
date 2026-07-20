use rsa::pss::SigningKey;
use rsa::signature::RandomizedSigner;
use rsa::RsaPrivateKey;

pub fn sign_data(key: &RsaPrivateKey, data: &[u8]) -> Vec<u8> {
    let signing_key = SigningKey::new(key.clone());
    signing_key.sign_with_rng(&mut rand::thread_rng(), data).to_vec()
}
