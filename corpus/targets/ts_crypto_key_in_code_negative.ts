// SAFE: keys loaded from environment variables
import { createCipheriv, randomBytes } from 'node:crypto';

function getEncryptionKey(): Buffer {
  const hex = process.env.ENCRYPTION_KEY;
  if (!hex || hex.length < 64) throw new Error('ENCRYPTION_KEY not configured');
  return Buffer.from(hex, 'hex');
}

function encryptApiKey(apiKey: string): string {
  const key = getEncryptionKey();
  const iv = randomBytes(16);
  const cipher = createCipheriv('aes-256-gcm', key, iv);
  const encrypted = cipher.update(apiKey, 'utf8') + cipher.final('hex');
  const tag = cipher.getAuthTag().toString('hex');
  return `${iv.toString('hex')}:${tag}:${encrypted}`;
}
