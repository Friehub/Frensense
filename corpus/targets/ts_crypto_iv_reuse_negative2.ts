// SAFE alternative: counter-based IV with unique nonce per key
import { createCipheriv, randomBytes } from 'node:crypto';

let counter = 0n;

function encrypt(plaintext: string, key: Buffer): string {
  const iv = Buffer.alloc(16);
  iv.writeBigUInt64BE(counter++, 8);
  randomBytes(8).copy(iv, 0);
  const cipher = createCipheriv('aes-256-cbc', key, iv);
  const encrypted = Buffer.concat([cipher.update(plaintext, 'utf8'), cipher.final()]);
  return `${iv.toString('hex')}:${encrypted.toString('hex')}`;
}
