// SAFE: Encrypt PII before storing in IndexedDB using a key derived from the auth token.

import { subtle } from 'node:crypto';

function getEncryptionKey(): Promise<CryptoKey> {
  const authToken = sessionStorage.getItem('authToken');
  if (!authToken) throw new Error('Not authenticated');
  return subtle.importKey(
    'raw',
    new TextEncoder().encode(authToken).slice(0, 32),
    'AES-GCM',
    false,
    ['encrypt', 'decrypt']
  );
}

async function encryptPII(plaintext: string, key: CryptoKey): Promise<{ iv: Uint8Array; ciphertext: ArrayBuffer }> {
  const iv = crypto.getRandomValues(new Uint8Array(12));
  const ciphertext = await subtle.encrypt(
    { name: 'AES-GCM', iv },
    key,
    new TextEncoder().encode(plaintext)
  );
  return { iv, ciphertext };
}

async function saveProfile(profile: { email: string; phone: string }): Promise<void> {
  const key = await getEncryptionKey();
  const encryptedEmail = await encryptPII(profile.email, key);
  const encryptedPhone = await encryptPII(profile.phone, key);
  const db = await openDatabase();
  const tx = db.transaction('profiles', 'readwrite');
  tx.objectStore('profiles').put({
    emailIv: encryptedEmail.iv,
    emailData: encryptedEmail.ciphertext,
    phoneIv: encryptedPhone.iv,
    phoneData: encryptedPhone.ciphertext,
  });
}

function openDatabase(): Promise<IDBDatabase> {
  return new Promise((resolve, reject) => {
    const req = indexedDB.open('UserData', 1);
    req.onupgradeneeded = () => {
      req.result.createObjectStore('profiles', { keyPath: 'id' });
    };
    req.onsuccess = () => resolve(req.result);
    req.onerror = () => reject(req.error);
  });
}
