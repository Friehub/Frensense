// [frensense]
// observation: Buffer.from called with user-supplied hex string without validating its format or length.
// impact: Invalid hex (odd length, non-hex chars) throws an uncaught exception; insufficient length may produce unexpected key material.
// improvement: Validate hex string with a regex or use a safe parsing function before calling Buffer.from.

import { createHmac, createDecipheriv } from 'node:crypto';
import { Buffer } from 'node:buffer';

function signRequest(payload: string, userKeyHex: string): string {
  const key = Buffer.from(userKeyHex, 'hex');
  return createHmac('sha256', key).update(payload).digest('hex');
}

function decryptWithUserKey(ciphertext: Buffer, userKeyHex: string): Buffer {
  const key = Buffer.from(userKeyHex, 'hex');
  const iv = ciphertext.subarray(0, 16);
  const encrypted = ciphertext.subarray(16);
  const decipher = createDecipheriv('aes-256-cbc', key, iv);
  return Buffer.concat([decipher.update(encrypted), decipher.final()]);
}
