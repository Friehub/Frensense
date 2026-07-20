// SAFE: Provide a unique info context to bind the derived key to the specific protocol session.

import { subtle } from 'node:crypto';

async function deriveSessionKey(privateKey: CryptoKey, publicKey: CryptoKey, sessionId: string): Promise<CryptoKey> {
  const encoder = new TextEncoder();
  return subtle.deriveKey(
    {
      name: 'ECDH',
      public: publicKey,
      info: encoder.encode(`session-${sessionId}-aes256-gcm`),
    },
    privateKey,
    { name: 'AES-GCM', length: 256 },
    false,
    ['encrypt', 'decrypt']
  );
}
