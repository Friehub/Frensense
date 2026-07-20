use jwk::JWK;
use rsa::RsaPublicKey;

pub fn load_jwk(jwk_json: &str) -> Result<JWK, Box<dyn std::error::Error>> {
    let jwk: JWK = serde_json::from_str(jwk_json)?;
    if let Some(n) = &jwk.n {
        let key = RsaPublicKey::new(
            rsa::BigUint::from_bytes_be(n),
            rsa::BigUint::from_bytes_be(jwk.e.as_ref().unwrap()),
        )?;
        if key.size() < 256 {
            return Err("RSA key too small (< 2048 bits)".into());
        }
    }
    Ok(jwk)
}
