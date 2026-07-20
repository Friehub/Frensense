// SAFE: Validates RSA modulus size before use, rejecting keys < 2048 bits.
use jwk::JWK;

fn validate_jwk(jwk: &JWK) -> Result<(), &'static str> {
    if jwk.kty == "RSA" {
        if let Some(ref n) = jwk.n {
            if n.len() < 256 {
                return Err("RSA key too small");
            }
        }
    }
    Ok(())
}

pub fn load_jwk_checked(jwk_json: &str) -> Result<JWK, Box<dyn std::error::Error>> {
    let jwk: JWK = serde_json::from_str(jwk_json)?;
    validate_jwk(&jwk)?;
    Ok(jwk)
}
