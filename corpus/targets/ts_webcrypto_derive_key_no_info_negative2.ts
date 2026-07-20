// SAFE: Use HKDF after ECDH to bind context information into the derived key.

import { subtle } from 'node:crypto';

async function deriveSessionKey(privateKey: CryptoKey, publicKey: CryptoKey, sessionId: string): Promise<CryptoKey> {
  const sharedBits = await subtle.deriveBits(
    { name: 'ECDH', public: publicKey },
    privateKey,
    256
  );

  const encoder = new TextEncoder();
  const baseKey = await subtle.importKey(
    'raw',
    sharedBits,
    'HKDF',
    false,
    ['deriveKey']
  );

  return subtle.deriveKey(
    {
      name: 'HKDF',
      hash: 'SHA-256',
      salt: encoder.encode(sessionId),
      info: encoder.encode('aes256-gcm-encryption'),
    },
    baseKey,
    { name: 'AES-GCM', length: 256 },
    false,
    ['encrypt', 'decrypt']
  );
}
