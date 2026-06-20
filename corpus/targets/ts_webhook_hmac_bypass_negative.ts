// Safe: uses constant-time comparison for HMAC
function verifyWebhookSignature(rawBody: string, signature: string): boolean {
    const hash = crypto
        .createHmac('sha512', WEBHOOK_SECRET)
        .update(rawBody)
        .digest('hex');
    return crypto.timingSafeEqual(Buffer.from(hash), Buffer.from(signature));
}

// Safe: constant-time comparison for API key validation
function validateApiKey(providedKey: string, expectedKey: string): boolean {
    const a = Buffer.from(providedKey);
    const b = Buffer.from(expectedKey);
    if (a.length !== b.length) return false;
    return crypto.timingSafeEqual(a, b);
}

// Safe: JWT verification uses built-in library verification
function verifyJwtToken(token: string, secret: string): boolean {
    try {
        jwt.verify(token, secret, { algorithms: ['HS256'] });
        return true;
    } catch {
        return false;
    }
}
