// SAFE: IV length validated before createCipheriv call
import { createCipheriv, randomBytes } from 'node:crypto';

function encrypt(plaintext: string, key: Buffer, iv: Buffer): Buffer {
  if (iv.length !== 16) {
    throw new Error('IV must be 16 bytes for AES-256-CBC');
  }
  const cipher = createCipheriv('aes-256-cbc', key, iv);
  return Buffer.concat([cipher.update(plaintext, 'utf8'), cipher.final()]);
}

function handleRequest(userIvHex: string, key: Buffer, data: string): Buffer {
  const iv = Buffer.from(userIvHex, 'hex');
  return encrypt(data, key, iv);
}
