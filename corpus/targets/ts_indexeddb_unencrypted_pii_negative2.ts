// SAFE: Store only a non-sensitive user identifier in IndexedDB and fetch PII from the server on demand.

async function getProfileFromServer(): Promise<{ email: string; phone: string }> {
  const response = await fetch('/api/profile', {
    credentials: 'include',
  });
  if (!response.ok) throw new Error('Failed to fetch profile');
  return response.json() as Promise<{ email: string; phone: string }>;
}

async function getProfile(): Promise<{ email: string; phone: string }> {
  const db = await openDatabase();
  const tx = db.transaction('cache', 'readonly');
  const cached = await new Promise<{ userId: number } | undefined>((resolve) => {
    const req = tx.objectStore('cache').get('user-info');
    req.onsuccess = () => resolve(req.result);
    req.onerror = () => resolve(undefined);
  });

  if (cached) {
    return getProfileFromServer();
  }

  return getProfileFromServer();
}

function openDatabase(): Promise<IDBDatabase> {
  return new Promise((resolve, reject) => {
    const req = indexedDB.open('AppCache', 1);
    req.onupgradeneeded = () => {
      req.result.createObjectStore('cache');
    };
    req.onsuccess = () => resolve(req.result);
    req.onerror = () => reject(req.error);
  });
}
