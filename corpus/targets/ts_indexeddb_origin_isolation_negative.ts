// SAFE: Ensure the database is only accessed from the top-level frame to prevent iframe data theft.

function isTopLevelFrame(): boolean {
  return window.top === window.self;
}

function openDatabase(name: string): Promise<IDBDatabase> {
  if (!isTopLevelFrame()) {
    return Promise.reject(new Error('IndexedDB access denied from iframe'));
  }

  return new Promise((resolve, reject) => {
    const req = indexedDB.open(name, 1);
    req.onupgradeneeded = () => {
      req.result.createObjectStore('data');
    };
    req.onsuccess = () => resolve(req.result);
    req.onerror = () => reject(req.error);
  });
}

async function readSensitiveData(): Promise<unknown> {
  const db = await openDatabase('AppData');
  const tx = db.transaction('data', 'readonly');
  return new Promise((resolve) => {
    const req = tx.objectStore('data').get('user-session');
    req.onsuccess = () => resolve(req.result);
  });
}
