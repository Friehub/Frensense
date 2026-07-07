// [frensense]
// observation: The function generates a sensitive token (e.g., session ID, password reset token) using Math.random() instead of a cryptographically secure pseudo-random number generator (CSPRNG).
// impact: Tokens generated with Math.random() are predictable. An attacker can determine the random number generator's internal state and predict past or future tokens, leading to session hijacking or account takeover.
// improvement: Use the Node.js crypto module (e.g., crypto.randomBytes) or the Web Crypto API (crypto.getRandomValues) to generate secure random tokens.
function generateResetToken() {
    const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    let token = '';
    for (let i = 0; i < 32; i++) {
        token += chars.charAt(Math.floor(Math.random() * chars.length));
    }
    return token;
}
