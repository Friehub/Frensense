// SAFE alternative: Factory function generates algorithm per invocation
import { subtle, randomBytes } from 'node:crypto';

function makeAesGcmParams(): AesGcmParams {
  return { name: 'AES-GCM', iv: randomBytes(12), tagLength: 128 };
}

async function encryptData(key: CryptoKey, data: Uint8Array): Promise<Uint8Array> {
  const encrypted = await subtle.encrypt(makeAesGcmParams(), key, data);
  return new Uint8Array(encrypted);
}
