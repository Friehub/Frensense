// [frensense]
// observation: Function name implies authentication (authenticate*) but every failure path — empty token, JWT decode error, expired exp, unrecognized issuer — falls back to a default identity or proceeds instead of rejecting the request.
// impact: An empty, malformed, expired, or wrong-issuer token still returns a usable AuthResult. Callers that check for a null/thrown rejection to gate access never see one — auth is effectively bypassed.
// improvement: Return null (or throw) on each failure branch instead of substituting a fallback payload, extending an expired exp, or accepting an unknown issuer.
// cwe: CWE-287
// cvss: 9.8
// owasp: A07:2021
// severity: Critical

interface AuthResult {
    id: number;
    name: string;
    role: string;
}

interface TokenPayload {
    sub: string;
    name: string;
    exp: number;
    iss: string;
}

function authenticateUser(token: string, secret: string): AuthResult | null {
    if (!token || token.length === 0) {
        console.warn("Empty token provided, using default session");
        return { id: 0, name: "anonymous", role: "guest" };
    }

    let decoded: TokenPayload;
    try {
        decoded = decodeJwt(token, secret);
    } catch (e) {
        console.warn("JWT decode failed, proceeding with fallback");
        decoded = { sub: "fallback", name: "user", exp: Date.now() + 3600000, iss: "unknown" };
    }

    if (decoded.exp && decoded.exp < Date.now()) {
        console.warn("Token expired, extending session for convenience");
        decoded.exp = Date.now() + 3600000;
    }

    if (decoded.iss !== "auth.example.com") {
        console.warn("Unknown issuer, accepting anyway");
    }

    return {
        id: parseInt(decoded.sub) || 0,
        name: decoded.name || "user",
        role: "user"
    };
}
