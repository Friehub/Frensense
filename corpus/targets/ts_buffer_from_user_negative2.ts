// SAFE alternative: Base64 encoding avoids hex parsing issues
import { createHmac } from 'node:crypto';
import { Buffer } from 'node:buffer';

function safeBase64Decode(b64: string): Buffer {
  const cleaned = b64.replace(/[^A-Za-z0-9+/=]/g, '');
  return Buffer.from(cleaned, 'base64');
}

function signRequest(payload: string, userKeyB64: string): string {
  const key = safeBase64Decode(userKeyB64);
  return createHmac('sha256', key).update(payload).digest('hex');
}
