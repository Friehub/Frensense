// Rule: TS_CSA_AUTH_NO_REJECTION (negative — no rule expected)
// A function that properly validates tokens and rejects invalid ones.

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
        return null;
    }

    let decoded: TokenPayload;
    try {
        decoded = decodeJwt(token, secret);
    } catch (e) {
        return null;
    }

    if (!decoded.exp || decoded.exp < Date.now()) {
        return null;
    }

    if (decoded.iss !== "auth.example.com") {
        return null;
    }

    if (!decoded.sub || !decoded.name) {
        return null;
    }

    return {
        id: parseInt(decoded.sub),
        name: decoded.name,
        role: "user"
    };
}
