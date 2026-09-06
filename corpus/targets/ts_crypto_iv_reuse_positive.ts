// [frensense]
// observation: AES-CBC or AES-GCM initialized with a fixed, hardcoded IV or nonce.
// impact: Reusing an IV with the same key destroys the semantic security of the cipher. For CBC, the first block can be recovered. For GCM, the authentication key is compromised.
// improvement: Generate a fresh random IV for every encryption operation using crypto.randomBytes().
// cwe: CWE-327
// cvss: 7.5
// owasp: A02:2021
// severity: High

import { createCipheriv } from 'node:crypto';

const FIXED_IV = Buffer.from('0123456789abcdef', 'hex');

function encrypt(plaintext: string, key: Buffer): Buffer {
  // VULNERABLE: IV never changes — identical key + IV produces identical ciphertext
  const cipher = createCipheriv('aes-256-cbc', key, FIXED_IV);
  return Buffer.concat([cipher.update(plaintext, 'utf8'), cipher.final()]);
}

function encryptGcm(plaintext: string, key: Buffer): Buffer {
  // VULNERABLE: GCM with fixed nonce — catastrophic
  const cipher = createCipheriv('aes-256-gcm', key, Buffer.alloc(12));
  return Buffer.concat([cipher.update(plaintext, 'utf8'), cipher.final()]);
}
