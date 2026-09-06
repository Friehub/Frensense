// SAFE alternative: keys from cloud secrets manager
import { createCipheriv, randomBytes } from 'node:crypto';

async function getEncryptionKey(): Promise<Buffer> {
  if (process.env.ENCRYPTION_KEY) return Buffer.from(process.env.ENCRYPTION_KEY, 'hex');
  const { getSecret } = await import('./secrets');
  const secret = await getSecret('encryption-key');
  return Buffer.from(secret, 'hex');
}

async function encryptApiKey(apiKey: string): Promise<string> {
  const key = await getEncryptionKey();
  const iv = randomBytes(16);
  const cipher = createCipheriv('aes-256-gcm', key, iv);
  const encrypted = cipher.update(apiKey, 'utf8') + cipher.final('hex');
  return `${iv.toString('hex')}:${cipher.getAuthTag().toString('hex')}:${encrypted}`;
}
