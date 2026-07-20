// SAFE: PBKDF2 with 600,000 iterations per OWASP 2023 recommendation
import { pbkdf2Sync, randomBytes } from 'node:crypto';

const ITERATIONS = 600_000;

function deriveKey(password: string, salt?: Buffer): Buffer {
  const s = salt ?? randomBytes(16);
  return pbkdf2Sync(password, s, ITERATIONS, 32, 'sha256');
}

function hashPassword(password: string): string {
  const salt = randomBytes(16);
  const key = pbkdf2Sync(password, salt, ITERATIONS, 64, 'sha512');
  return `${salt.toString('hex')}:${key.toString('hex')}`;
}
