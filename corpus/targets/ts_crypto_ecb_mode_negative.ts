// SAFE: AES-256-GCM with random IV — authenticated encryption
import { createCipheriv, createDecipheriv, randomBytes } from 'node:crypto';

const ALGORITHM = 'aes-256-gcm';

function encryptData(plaintext: string, key: Buffer): { ciphertext: Buffer; iv: Buffer; tag: Buffer } {
  const iv = randomBytes(16);
  const cipher = createCipheriv(ALGORITHM, key, iv);
  const encrypted = Buffer.concat([cipher.update(plaintext, 'utf8'), cipher.final()]);
  const tag = cipher.getAuthTag();
  return { ciphertext: encrypted, iv, tag };
}

function decryptData(enc: { ciphertext: Buffer; iv: Buffer; tag: Buffer }, key: Buffer): string {
  const decipher = createDecipheriv(ALGORITHM, key, enc.iv);
  decipher.setAuthTag(enc.tag);
  return decipher.update(enc.ciphertext) + decipher.final('utf8');
}
