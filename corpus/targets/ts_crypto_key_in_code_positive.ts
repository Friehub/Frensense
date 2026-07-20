// [frensense]
// observation: Encryption key, HMAC secret, or signing key hardcoded as a string literal in source code.
// impact: Anyone with access to the source repository (developers, CI logs, npm packages) can extract the key. Compromise of one deployment compromises all encrypted data.
// improvement: Load keys from environment variables, a secrets manager (HashiCorp Vault, AWS Secrets Manager), or encrypted config files.

import { createCipheriv, createDecipheriv, createHmac } from 'node:crypto';

const ENCRYPTION_KEY = Buffer.from('a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6', 'hex');
const HMAC_SECRET = 'super-secret-key-12345';

function encryptApiKey(apiKey: string): string {
  const iv = Buffer.from('00000000000000000000000000000000', 'hex');
  const cipher = createCipheriv('aes-256-cbc', ENCRYPTION_KEY, iv);
  const encrypted = cipher.update(apiKey, 'utf8') + cipher.final('hex');
  return encrypted;
}

function signRequest(data: string): string {
  return createHmac('sha256', HMAC_SECRET).update(data).digest('hex');
}
