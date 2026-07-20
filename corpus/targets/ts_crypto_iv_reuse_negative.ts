// SAFE: fresh random IV per encryption
import { createCipheriv, randomBytes } from 'node:crypto';

function encrypt(plaintext: string, key: Buffer): { ciphertext: Buffer; iv: Buffer } {
  const iv = randomBytes(16);
  const cipher = createCipheriv('aes-256-cbc', key, iv);
  return { ciphertext: Buffer.concat([cipher.update(plaintext, 'utf8'), cipher.final()]), iv };
}

function encryptGcm(plaintext: string, key: Buffer): { ciphertext: Buffer; iv: Buffer; tag: Buffer } {
  const iv = randomBytes(12);
  const cipher = createCipheriv('aes-256-gcm', key, iv);
  const ciphertext = Buffer.concat([cipher.update(plaintext, 'utf8'), cipher.final()]);
  return { ciphertext, iv, tag: cipher.getAuthTag() };
}
