// SAFE: Generate keys as non-extractable so private key material never leaves the secure context.

import { subtle } from 'node:crypto';

async function generateAndRegisterKey(): Promise<void> {
  const keyPair = await subtle.generateKey(
    { name: 'ECDSA', namedCurve: 'P-256' },
    false,
    ['sign', 'verify']
  );

  const exported = await subtle.exportKey('spki', keyPair.publicKey);
  const base64PubKey = Buffer.from(exported).toString('base64');

  await fetch('/api/register-key', {
    method: 'POST',
    body: JSON.stringify({ publicKey: base64PubKey }),
    headers: { 'Content-Type': 'application/json' },
  });
}
