// SAFE: Convert the verify result into a rejected promise for use in promise chains.

import { subtle } from 'node:crypto';

async function verifyOrReject(
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
    return Promise.reject(new Error('Signature verification failed'));
  }
}

async function processSignedMessage(
  publicKey: CryptoKey,
  signature: ArrayBuffer,
  data: ArrayBuffer
): Promise<void> {
  await verifyOrReject(publicKey, signature, data);
  applyToLedger(data);
}

function applyToLedger(_data: ArrayBuffer): void {
  // process data
}
