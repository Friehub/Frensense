// SAFE: Use a per-origin isolation key in the database name to prevent cross-frame data sharing.

function getIsolationKey(): string {
  const origin = window.location.origin;
  const topOrigin = window.top?.location.origin ?? origin;
  if (origin !== topOrigin) {
    throw new Error('Cross-origin frame access denied');
  }
  return topOrigin;
}

function openDatabase(): Promise<IDBDatabase> {
  const key = getIsolationKey();
  const dbName = `AppData_${key}`;

  return new Promise((resolve, reject) => {
    const req = indexedDB.open(dbName, 1);
    req.onupgradeneeded = () => {
      req.result.createObjectStore('data');
    };
    req.onsuccess = () => resolve(req.result);
    req.onerror = () => reject(req.error);
  });
}

async function readSensitiveData(): Promise<unknown> {
  const db = await openDatabase();
  const tx = db.transaction('data', 'readonly');
  return new Promise((resolve) => {
    const req = tx.objectStore('data').get('user-session');
    req.onsuccess = () => resolve(req.result);
  });
}
