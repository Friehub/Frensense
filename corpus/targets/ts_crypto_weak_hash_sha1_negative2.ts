// SAFE alternative: SHA-3 via built-in or blake2
import { createHash, createHmac } from 'node:crypto';

function hashFile(fileBuffer: Buffer): string {
  return createHash('sha3-256').update(fileBuffer).digest('hex');
}

function signData(data: string, key: string): string {
  return createHmac('sha384', key).update(data).digest('hex');
}
