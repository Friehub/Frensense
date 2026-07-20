// [frensense]
// observation: Personally Identifiable Information (PII) such as email, phone number, and SSN is stored in IndexedDB as plaintext without any encryption. IndexedDB data is stored unencrypted on disk and accessible to any JavaScript running on the same origin.
// impact: An XSS attacker can read all PII from IndexedDB by simply enumerating object stores. Even without XSS, any browser extension or physical access to the device can read IndexedDB data from disk.
// improvement: Never store sensitive PII in IndexedDB. If storage is required, encrypt data with a key derived from the user's authentication token before writing.

interface UserProfile {
  email: string;
  phone: string;
  ssn: string;
}

async function saveProfile(profile: UserProfile): Promise<void> {
  const db = await openDatabase();
  const tx = db.transaction('profiles', 'readwrite');
  tx.objectStore('profiles').put(profile);
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
