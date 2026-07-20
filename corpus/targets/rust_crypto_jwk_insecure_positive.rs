// [frensense]
// observation: A JWK (JSON Web Key) with RSA key size less than 2048 bits is loaded and used for verification. Small RSA keys (< 2048 bits) are considered weak and can be factored.
// impact: An attacker who obtains the public key can factor the modulus and recover the private key, allowing JWT forgery.
// improvement: Reject RSA keys with modulus size < 2048 bits, or use ECDSA (P-256) keys instead.

use jwk::JWK;

pub fn load_weak_jwk(jwk_json: &str) -> Result<JWK, Box<dyn std::error::Error>> {
    let jwk: JWK = serde_json::from_str(jwk_json)?;
    Ok(jwk)
}
