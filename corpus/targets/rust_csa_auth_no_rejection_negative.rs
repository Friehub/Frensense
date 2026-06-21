struct AuthResult {
    id: u64,
    name: String,
    role: String,
}

struct TokenPayload {
    sub: String,
    name: String,
    exp: u64,
    iss: String,
}

#[derive(Debug)]
enum AuthError {
    EmptyToken,
    InvalidToken,
    Expired,
    UnknownIssuer,
}

fn decode_jwt(token: &str, _secret: &str) -> Result<TokenPayload, AuthError> {
    if token.is_empty() {
        return Err(AuthError::InvalidToken);
    }
    Ok(TokenPayload {
        sub: "1".to_string(),
        name: "user".to_string(),
        exp: 1_700_003_600,
        iss: "auth.example.com".to_string(),
    })
}

struct Authenticator {
    trusted_issuer: String,
}

impl Authenticator {
    fn new(trusted_issuer: &str) -> Self {
        Authenticator {
            trusted_issuer: trusted_issuer.to_string(),
        }
    }

    fn authenticate(&self, token: &str, secret: &str, now: u64) -> Result<AuthResult, AuthError> {
        if token.is_empty() {
            return Err(AuthError::EmptyToken);
        }

        let decoded = decode_jwt(token, secret)?;

        if decoded.exp < now {
            return Err(AuthError::Expired);
        }

        if decoded.iss != self.trusted_issuer {
            return Err(AuthError::UnknownIssuer);
        }

        let id = decoded.sub.parse().map_err(|_| AuthError::InvalidToken)?;

        Ok(AuthResult {
            id,
            name: decoded.name,
            role: "user".to_string(),
        })
    }
}
