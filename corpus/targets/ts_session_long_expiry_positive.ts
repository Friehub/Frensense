// [frensense]
// observation: JWT or session cookie is issued with an excessively long expiry (365 days or no expiry at all) with no revocation mechanism.
// impact: A stolen token remains valid for an entire year. Without a revocation list, there is no way to invalidate a compromised session.
// improvement: Use short expiry (15-60 minutes) combined with refresh tokens, or at most 7 days for long-lived sessions with a revocation mechanism.

import jwt from 'jsonwebtoken';

export function issueToken(userId: string): string {
  return jwt.sign({ sub: userId, role: 'user' }, process.env.JWT_SECRET!, { expiresIn: '365d' });
}

export function createSession(userId: string): string {
  return jwt.sign({ sub: userId }, 'secret-key', { expiresIn: '9999y' });
}
