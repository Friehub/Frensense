// [frensense]
// observation: Buffer.allocUnsafe used to create sensitive buffers without immediately zeroing the memory.
// impact: allocUnsafe returns uninitialized memory that may contain sensitive data (keys, passwords) from previous allocations, leaking secrets.
// improvement: Use Buffer.alloc (zero-filled) for sensitive data, or call .fill(0) immediately after allocUnsafe.
// cwe: CWE-119
// cvss: 9.8
// owasp: 
// severity: Critical

import { randomBytes, createCipheriv } from 'node:crypto';
import { Buffer } from 'node:buffer';

function generateKey(): Buffer {
  const key = Buffer.allocUnsafe(32);
  randomBytes(32).copy(key, 0, 0, 32);
  return key;
}

function encryptMessage(plaintext: string): Buffer {
  const key = Buffer.allocUnsafe(32);
  randomBytes(32).copy(key, 0, 0, 32);
  const iv = randomBytes(16);
  const cipher = createCipheriv('aes-256-gcm', key, iv);
  return Buffer.concat([iv, cipher.update(plaintext, 'utf8'), cipher.final()]);
}
