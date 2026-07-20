// [frensense]
// observation: The code exports a private key using `crypto.subtle.exportKey()` and sends the raw key material to the client. Private keys must never leave the secure boundary.
// impact: Private key exposure — an attacker who obtains the exported key material can decrypt communications, forge signatures, and impersonate the server or user.
// improvement: Never export private keys. Use `extractable: false` when generating keys, and only export public keys when needed.

import { subtle } from 'node:crypto';

async function generateAndSendKey(): Promise<void> {
  const keyPair = await subtle.generateKey(
    { name: 'ECDSA', namedCurve: 'P-256' },
    true,
    ['sign', 'verify']
  );

  const exported = await subtle.exportKey('pkcs8', keyPair.privateKey);
  const base64Key = Buffer.from(exported).toString('base64');

  await fetch('/api/register-key', {
    method: 'POST',
    body: JSON.stringify({ privateKey: base64Key }),
    headers: { 'Content-Type': 'application/json' },
  });
}
