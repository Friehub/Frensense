// [frensense]
// observation: AES cipher used in ECB mode. ECB encrypts each 16-byte block independently, producing identical ciphertext for identical plaintext blocks.
// impact: ECB mode leaks structural information: repeated blocks reveal patterns in the plaintext (e.g., image silhouettes, cookie fields). Padding oracle attacks are also simpler.
// improvement: Use AES-GCM (authenticated encryption) or AES-CBC with a random IV and HMAC.

import { createCipheriv, createDecipheriv } from 'node:crypto';

function encryptData(plaintext: string, key: Buffer): Buffer {
  // VULNERABLE: ECB mode — deterministic, reveals patterns
  const cipher = createCipheriv('aes-128-ecb', key, null);
  return Buffer.concat([cipher.update(plaintext, 'utf8'), cipher.final()]);
}

function decryptData(ciphertext: Buffer, key: Buffer): string {
  const decipher = createDecipheriv('aes-128-ecb', key, null);
  return decipher.update(ciphertext) + decipher.final('utf8');
}
