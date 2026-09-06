// SAFE alternative: Reject IV outside expected range before cipher creation
import { createCipheriv, randomBytes } from 'node:crypto';

const EXPECTED_IV_LENGTH = 16;

function encrypt(plaintext: string, key: Buffer, iv: Buffer): Buffer {
  if (iv.byteLength !== EXPECTED_IV_LENGTH) {
    throw new RangeError(`IV must be exactly ${EXPECTED_IV_LENGTH} bytes`);
  }
  const cipher = createCipheriv('aes-256-cbc', key, iv);
  return Buffer.concat([cipher.update(plaintext, 'utf8'), cipher.final()]);
}

function handleRequest(userIvHex: string, key: Buffer, data: string): Buffer {
  const iv = Buffer.from(userIvHex, 'hex');
  return encrypt(data, key, iv);
}
