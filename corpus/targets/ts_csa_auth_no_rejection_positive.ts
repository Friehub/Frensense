// Rule: TS_CSA_AUTH_NO_REJECTION
// A function that looks like it validates tokens but actually accepts everything.

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
