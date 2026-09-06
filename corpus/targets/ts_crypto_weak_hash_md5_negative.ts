// SAFE: bcrypt for password storage, HMAC-SHA256 for signatures
import { createHmac, randomBytes } from 'node:crypto';
import bcrypt from 'bcrypt';

async function hashPassword(password: string): Promise<string> {
  const saltRounds = 12;
  return bcrypt.hash(password, saltRounds);
}

function signPayload(payload: object, secret: string): string {
  const hmac = createHmac('sha256', secret);
  hmac.update(JSON.stringify(payload));
  return hmac.digest('hex');
}

function verifySignature(sig: string, payload: object, secret: string): boolean {
  const expected = signPayload(payload, secret);
  return sig.length === expected.length && crypto.timingSafeEqual(Buffer.from(sig), Buffer.from(expected));
}
