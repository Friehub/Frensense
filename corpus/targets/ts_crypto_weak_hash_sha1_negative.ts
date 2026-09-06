// SAFE: SHA-256 for integrity, HMAC-SHA256 for signatures
import { createHash, createHmac, timingSafeEqual } from 'node:crypto';

function hashFile(fileBuffer: Buffer): string {
  return createHash('sha256').update(fileBuffer).digest('hex');
}

function signData(data: string, key: string): string {
  return createHmac('sha256', key).update(data).digest('hex');
}

function verifyChecksum(storedChecksum: string, data: Buffer): boolean {
  const computed = hashFile(data);
  if (storedChecksum.length !== computed.length) return false;
  return timingSafeEqual(Buffer.from(storedChecksum), Buffer.from(computed));
}
