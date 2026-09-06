// SAFE alternative: Use scrypt instead of PBKDF2 (memory-hard)
import { scryptSync, randomBytes } from 'node:crypto';

function hashPassword(password: string): string {
  const salt = randomBytes(32);
  const key = scryptSync(password, salt, 64, { N: 16384, r: 8, p: 1 });
  return `${salt.toString('hex')}:${key.toString('hex')}`;
}
