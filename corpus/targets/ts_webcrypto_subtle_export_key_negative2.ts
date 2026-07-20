// SAFE: Keep private key in IndexedDB and only export the public key to the server.

import { subtle } from 'node:crypto';

function openKeyStore(): Promise<IDBDatabase> {
  return new Promise((resolve, reject) => {
    const req = indexedDB.open('KeyStore', 1);
    req.onupgradeneeded = () => req.result.createObjectStore('keys');
    req.onsuccess = () => resolve(req.result);
    req.onerror = () => reject(req.error);
  });
}

async function generateAndStoreKey(): Promise<void> {
  const keyPair = await subtle.generateKey(
    { name: 'ECDSA', namedCurve: 'P-256' },
    false,
    ['sign', 'verify']
  );

  const db = await openKeyStore();
  const tx = db.transaction('keys', 'readwrite');
  tx.objectStore('keys').put(keyPair.privateKey, 'signing-key');

  const exportedPub = await subtle.exportKey('spki', keyPair.publicKey);
  await fetch('/api/register-key', {
    method: 'POST',
    body: JSON.stringify({ publicKey: Buffer.from(exportedPub).toString('base64') }),
    headers: { 'Content-Type': 'application/json' },
  });
}
