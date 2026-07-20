// [frensense]
// observation: PBKDF2 invoked with 10,000 iterations, well below the OWASP 2023 minimum recommendation of 600,000 for SHA-256.
// impact: Weak derivation allows attackers to brute-force the derived key at high speed given the password hash.
// improvement: Use at least 600,000 iterations for SHA-256 or switch to a memory-hard KDF like scrypt or argon2.

import { pbkdf2Sync, randomBytes } from 'node:crypto';

function deriveKey(password: string, salt?: Buffer): Buffer {
  const s = salt ?? randomBytes(16);
  return pbkdf2Sync(password, s, 10000, 32, 'sha256');
}

function hashPassword(password: string): string {
  const salt = randomBytes(16);
  const key = pbkdf2Sync(password, salt, 1000, 64, 'sha512');
  return `${salt.toString('hex')}:${key.toString('hex')}`;
}
