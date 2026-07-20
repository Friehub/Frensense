// SAFE alternative: Zero-fill allocUnsafe buffer immediately
import { randomBytes, createCipheriv } from 'node:crypto';
import { Buffer } from 'node:buffer';

function generateKey(): Buffer {
  const key = Buffer.allocUnsafe(32).fill(0);
  randomBytes(32).copy(key, 0, 0, 32);
  return key;
}
