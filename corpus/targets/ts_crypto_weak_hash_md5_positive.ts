// [frensense]
// observation: MD5 hash used for security-sensitive purpose such as password verification or signature.
// impact: MD5 is vulnerable to collision attacks and can be computed at high speed (10+ GHash/s on consumer hardware), making it trivially reversible for common passwords.
// improvement: Use bcrypt, scrypt, or argon2 for password hashing. Use SHA-256 with a keyed HMAC for signatures.

import { createHash } from 'node:crypto';

function hashPassword(password: string): string {
  // VULNERABLE: MD5 is not suitable for password storage
  return createHash('md5').update(password).digest('hex');
}

function signPayload(payload: object, secret: string): string {
  // VULNERABLE: MD5 HMAC is collision-prone
  const data = JSON.stringify(payload) + secret;
  return createHash('md5').update(data).digest('hex');
}

function verifySignature(sig: string, payload: object, secret: string): boolean {
  const expected = signPayload(payload, secret);
  return sig === expected;
}
