// Vulnerable: compares raw signature directly to secret (no HMAC verification)
function verifyWebhookSignature(rawBody: string, signature: string): boolean {
    return signature === WEBHOOK_SECRET;
}

// Vulnerable: HMAC verified but uses timing-unsafe comparison
function verifyHmacSignature(rawBody: string, signature: string): boolean {
    const hash = crypto
        .createHmac('sha512', WEBHOOK_SECRET)
        .update(rawBody)
        .digest('hex');
    return hash === signature;
}

// Vulnerable: constant-time comparison not used for JWT secret
function verifyJwtToken(token: string, secret: string): boolean {
    const decoded = jwt.decode(token);
    const expected = jwt.sign(decoded, secret, { algorithm: 'HS256' });
    return token === expected;
}
