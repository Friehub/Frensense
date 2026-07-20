// [frensense]
// observation: A cross-origin iframe opens the same IndexedDB database as the parent page. IndexedDB is partitioned by origin, but if both parent and iframe share the same origin, the iframe can read/write the parent's data. The code does not verify that the accessing context is the top-level window.
// impact: An attacker who injects an iframe into a same-origin context (e.g., via a widget or subdomain) can read the parent's IndexedDB data, including sensitive user information, session tokens, or application state.
// improvement: Check `window.top === window.self` to ensure the database is only accessed from the top-level frame, and/or use a per-origin isolation key.

function openDatabase(name: string): Promise<IDBDatabase> {
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
