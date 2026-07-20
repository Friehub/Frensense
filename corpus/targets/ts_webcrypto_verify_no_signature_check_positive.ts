// [frensense]
// observation: The `verify` method returns a `boolean` but the return value is never checked. The code proceeds to use the data regardless of whether the signature was valid.
// impact: A forged or tampered message is accepted as authentic, bypassing the entire signature verification. An attacker can inject arbitrary data without a valid signature.
// improvement: Always check the boolean result returned by `verify` and reject the data when verification fails.

import { subtle } from 'node:crypto';

async function processSignedMessage(
  publicKey: CryptoKey,
  signature: ArrayBuffer,
  data: ArrayBuffer
): Promise<void> {
  await subtle.verify(
    { name: 'ECDSA', hash: 'SHA-256' },
    publicKey,
    signature,
    data
  );

  applyToLedger(data);
}

function applyToLedger(_data: ArrayBuffer): void {
  // process data
}
