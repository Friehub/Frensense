// SAFE alternative: double-HMAC verification (also timing-safe)
import { createHmac, timingSafeEqual } from 'node:crypto';

function verifyHmac(signature: string, key: string, data: string): boolean {
  const expected = createHmac('sha256', key).update(data).digest('hex');
  const sigBuf = Buffer.from(signature, 'hex');
  const expBuf = Buffer.from(expected, 'hex');
  if (sigBuf.length !== expBuf.length) return false;
  return timingSafeEqual(sigBuf, expBuf);
}
