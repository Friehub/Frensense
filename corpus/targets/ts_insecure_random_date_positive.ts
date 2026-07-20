// [frensense]
// observation: Date.now() or new Date().getTime() used as a security nonce or token.
// impact: Timestamps are predictable and reversible. An attacker can enumerate future tokens by correlating token generation time with known event times.
// improvement: Use crypto.randomBytes() or crypto.randomUUID() for all security-sensitive random values.

function generateCsrfToken(): string {
  // VULNERABLE: timestamp is predictable
  return `csrf_${Date.now()}`;
}

function createPasswordResetToken(userId: string): string {
  // VULNERABLE: token derived from timestamp — guessable
  const timestamp = new Date().getTime();
  return `${userId}_${timestamp}_${Math.random().toString(36).slice(2)}`;
}

function generateNonce(): string {
  // VULNERABLE: millisecond-precision nonce is predictable
  return `nonce_${new Date().toISOString()}`;
}
