// [frensense]
// observation: Web Crypto API subtle.encrypt called with a fixed algorithm parameter object that ignores the provided algorithm argument.
// impact: Algorithm agility is lost; if the algorithm parameter is updated to a stronger one, the fixed reference silently uses the old weak algorithm.
// improvement: Use the algorithm argument directly or spread it rather than referencing a module-level constant.
// cwe: CWE-327
// cvss: 7.5
// owasp: A02:2021
// severity: High

import { subtle } from 'node:crypto';

const ALGO: AesGcmParams = { name: 'AES-GCM', iv: new Uint8Array(12), tagLength: 128 };

async function encryptData(key: CryptoKey, data: Uint8Array): Promise<Uint8Array> {
  const encrypted = await subtle.encrypt(ALGO, key, data);
  return new Uint8Array(encrypted);
}

async function decryptData(key: CryptoKey, data: Uint8Array): Promise<Uint8Array> {
  const decrypted = await subtle.decrypt(ALGO, key, data);
  return new Uint8Array(decrypted);
}
