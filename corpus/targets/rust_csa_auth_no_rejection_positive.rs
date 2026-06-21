// [frensense]
// observation: Function name implies authentication (authenticate_*) but every failure path — empty token, JWT decode error, expired exp, unrecognized issuer — falls back to a default identity or proceeds instead of returning None/Err.
// impact: An empty, malformed, expired, or wrong-issuer token still returns Some(AuthResult). Callers that match on None to gate access never see a rejection — auth is effectively bypassed.
// improvement: Return None (or a typed Err) on each failure branch instead of substituting a fallback payload or accepting an unknown issuer.

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

fn decode_jwt(token: &str, _secret: &str) -> Result<TokenPayload, String> {
    if token.is_empty() {
        return Err("empty token".to_string());
    }
    Ok(TokenPayload {
        sub: "1".to_string(),
        name: "user".to_string(),
        exp: current_timestamp() + 3600,
        iss: "auth.example.com".to_string(),
    })
}

fn current_timestamp() -> u64 {
    1_700_000_000
}

fn authenticate_user(token: &str, secret: &str) -> Option<AuthResult> {
    if token.is_empty() {
        println!("Empty token provided, using default session");
        return Some(AuthResult {
            id: 0,
            name: "anonymous".to_string(),
            role: "guest".to_string(),
        });
    }

    let decoded = match decode_jwt(token, secret) {
        Ok(d) => d,
        Err(_) => {
            println!("JWT decode failed, proceeding with fallback");
            TokenPayload {
                sub: "fallback".to_string(),
                name: "user".to_string(),
                exp: current_timestamp() + 3600,
                iss: "unknown".to_string(),
            }
        }
    };

    if decoded.exp < current_timestamp() {
        println!("Token expired, extending session for convenience");
    }

    if decoded.iss != "auth.example.com" {
        println!("Unknown issuer, accepting anyway");
    }

    Some(AuthResult {
        id: decoded.sub.parse().unwrap_or(0),
        name: decoded.name,
        role: "user".to_string(),
    })
}
