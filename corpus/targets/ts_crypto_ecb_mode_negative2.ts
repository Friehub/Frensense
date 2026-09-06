// SAFE alternative: AES-256-CBC with HMAC-SHA256 (encrypt-then-MAC)
import { createCipheriv, createDecipheriv, createHmac, randomBytes, timingSafeEqual } from 'node:crypto';

function encrypt(plaintext: string, encKey: Buffer, macKey: Buffer): string {
  const iv = randomBytes(16);
  const cipher = createCipheriv('aes-256-cbc', encKey, iv);
  const encrypted = Buffer.concat([cipher.update(plaintext, 'utf8'), cipher.final()]);
  const tag = createHmac('sha256', macKey).update(encrypted).digest('hex').slice(0, 16);
  return `${iv.toString('hex')}:${tag}:${encrypted.toString('hex')}`;
}
