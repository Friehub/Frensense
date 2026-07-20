// [frensense]
// observation: `deriveKey` is called `without` the `info` parameter in ECDH key derivation. The `info` parameter binds the derived key to a specific context, preventing the same shared secret from being used across different protocol contexts.
// impact: Without context binding, the same derived key can be reused across different protocol sessions or purposes, enabling cross-protocol attacks and weakening the cryptographic binding.
// improvement: Always provide a unique `info` string that identifies the protocol context, session, and purpose.

import { subtle } from 'node:crypto';

async function deriveSessionKey(privateKey: CryptoKey, publicKey: CryptoKey): Promise<CryptoKey> {
  return subtle.deriveKey(
    { name: 'ECDH', public: publicKey },
    privateKey,
    { name: 'AES-GCM', length: 256 },
    false,
    ['encrypt', 'decrypt']
  );
}
