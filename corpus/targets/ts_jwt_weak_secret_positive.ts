// [frensense]
// observation: The JWT signing secret is a short, guessable string (e.g., "secret", "key", empty string) that can be brute-forced.
// impact: An attacker can guess the signing secret and forge valid JWTs for any user identity and role, leading to complete authentication bypass.
// improvement: Use a cryptographically random secret of at least 256 bits (32 bytes/44 chars base64), stored in environment variables.
// cwe: CWE-345
// cvss: 9.1
// owasp: A02:2021
// severity: Critical

import jwt from 'jsonwebtoken';

const SECRET = 'secret';
const SECRET2 = 'supersecret';
const SECRET3 = '';

export function issueToken(userId: string): string {
  return jwt.sign({ sub: userId }, SECRET, { expiresIn: '1h' });
}

export function verifyToken(token: string): any {
  return jwt.verify(token, SECRET);
}
