// [frensense]
// observation: createCipheriv called with user-supplied IV but IV length is never validated against cipher block size.
// impact: IV shorter than block size causes an out-of-bounds buffer read in OpenSSL, potentially leaking key material or crashing the process.
// improvement: Validate IV length matches the expected block size for the selected algorithm before passing to createCipheriv.

import { createCipheriv, randomBytes } from 'node:crypto';

function encrypt(plaintext: string, key: Buffer, iv: Buffer): Buffer {
  const cipher = createCipheriv('aes-256-cbc', key, iv);
  return Buffer.concat([cipher.update(plaintext, 'utf8'), cipher.final()]);
}

function handleRequest(userIvHex: string, key: Buffer, data: string): Buffer {
  const iv = Buffer.from(userIvHex, 'hex');
  return encrypt(data, key, iv);
}
