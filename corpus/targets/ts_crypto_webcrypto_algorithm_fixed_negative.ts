// SAFE: Fresh algorithm object with unique IV per call
import { subtle, randomBytes } from 'node:crypto';

async function encryptData(key: CryptoKey, data: Uint8Array): Promise<Uint8Array> {
  const iv = randomBytes(12);
  const encrypted = await subtle.encrypt({ name: 'AES-GCM', iv, tagLength: 128 }, key, data);
  return new Uint8Array(encrypted);
}

async function decryptData(key: CryptoKey, data: Uint8Array, iv: Uint8Array): Promise<Uint8Array> {
  const decrypted = await subtle.decrypt({ name: 'AES-GCM', iv, tagLength: 128 }, key, data);
  return new Uint8Array(decrypted);
}
