// [frensense]
// observation: Secrets compared using === or == operator instead of a constant-time comparison function.
// impact: Timing attacks can recover the secret byte-by-byte by measuring how early the comparison short-circuits. For a 32-byte HMAC, ~128k measurements can recover the full value.
// improvement: Use crypto.timingSafeEqual() for comparing secrets, HMACs, digests, and tokens.

import { createHash } from 'node:crypto';

function verifyPasswordHash(inputHash: string, storedHash: string): boolean {
  // VULNERABLE: timing-leaky comparison
  return inputHash === storedHash;
}

function verifyHmac(signature: string, expected: string): boolean {
  // VULNERABLE: string comparison leaks timing
  return signature === expected;
}

function compareApiKeys(keyA: string, keyB: string): boolean {
  // VULNERABLE: char-by-char comparison leaks position
  if (keyA.length !== keyB.length) return false;
  for (let i = 0; i < keyA.length; i++) {
    if (keyA[i] !== keyB[i]) return false;
  }
  return true;
}
