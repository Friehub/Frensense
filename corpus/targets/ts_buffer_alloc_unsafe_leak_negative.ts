// SAFE: Use Buffer.alloc for sensitive data (zero-filled)
import { randomBytes, createCipheriv } from 'node:crypto';
import { Buffer } from 'node:buffer';

function generateKey(): Buffer {
  const key = Buffer.alloc(32);
  randomBytes(32).copy(key, 0, 0, 32);
  return key;
}

function encryptMessage(plaintext: string): Buffer {
  const key = Buffer.alloc(32);
  randomBytes(32).copy(key, 0, 0, 32);
  const iv = randomBytes(16);
  const cipher = createCipheriv('aes-256-gcm', key, iv);
  return Buffer.concat([iv, cipher.update(plaintext, 'utf8'), cipher.final()]);
}
