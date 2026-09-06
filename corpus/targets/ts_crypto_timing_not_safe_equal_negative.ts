// SAFE: constant-time comparison using crypto.timingSafeEqual
import { timingSafeEqual } from 'node:crypto';

function verifyPasswordHash(inputHash: string, storedHash: string): boolean {
  if (inputHash.length !== storedHash.length) return false;
  return timingSafeEqual(Buffer.from(inputHash), Buffer.from(storedHash));
}

function verifyHmac(signature: string, expected: string): boolean {
  if (signature.length !== expected.length) return false;
  return timingSafeEqual(Buffer.from(signature), Buffer.from(expected));
}

function compareApiKeys(keyA: string, keyB: string): boolean {
  if (keyA.length !== keyB.length) return false;
  return timingSafeEqual(Buffer.from(keyA), Buffer.from(keyB));
}
