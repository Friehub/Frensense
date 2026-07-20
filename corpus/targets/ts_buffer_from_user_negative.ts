// SAFE: Validate hex string before Buffer.from
import { createHmac } from 'node:crypto';
import { Buffer } from 'node:buffer';

const HEX_REGEX = /^[0-9a-fA-F]+$/;

function parseHexOrThrow(hex: string): Buffer {
  if (hex.length === 0 || hex.length % 2 !== 0 || !HEX_REGEX.test(hex)) {
    throw new Error('Invalid hex string');
  }
  return Buffer.from(hex, 'hex');
}

function signRequest(payload: string, userKeyHex: string): string {
  const key = parseHexOrThrow(userKeyHex);
  return createHmac('sha256', key).update(payload).digest('hex');
}
