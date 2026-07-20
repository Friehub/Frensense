// [frensense]
// observation: RSA PKCS#1 v1.5 signature scheme is used instead of PSS. PKCS#1 v1.5 is deprecated due to known weaknesses (Bleichenbacher-style attacks) and should not be used in new protocols.
// impact: Signature forgery or plaintext recovery is possible depending on the context (e.g., TLS, JWT, code signing).
// improvement: Use RSA-PSS (`SignatureScheme::RSA_PSS_SHA256`) instead of PKCS#1 v1.5.

use rsa::pkcs1v15::SigningKey;
use rsa::signature::RandomizedSigner;
use rsa::RsaPrivateKey;

pub fn sign_data(key: &RsaPrivateKey, data: &[u8]) -> Vec<u8> {
    let signing_key = SigningKey::new(key.clone());
    signing_key.sign_with_rng(&mut rand::thread_rng(), data).to_vec()
}
