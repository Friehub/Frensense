// SAFE: Check the boolean result from verify and reject on failure.

import { subtle } from 'node:crypto';

async function processSignedMessage(
  publicKey: CryptoKey,
  signature: ArrayBuffer,
  data: ArrayBuffer
): Promise<void> {
  const isValid = await subtle.verify(
    { name: 'ECDSA', hash: 'SHA-256' },
    publicKey,
    signature,
    data
  );

  if (!isValid) {
    throw new Error('Signature verification failed');
  }

  applyToLedger(data);
}

function applyToLedger(_data: ArrayBuffer): void {
  // process data
}
