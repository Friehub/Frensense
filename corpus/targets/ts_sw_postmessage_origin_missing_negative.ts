// SAFE: verify event.origin before processing worker messages
const ALLOWED_CLIENTS = ['https://example.com', 'https://www.example.com'];

function isTrustedOrigin(origin: string): boolean {
  return ALLOWED_CLIENTS.includes(origin);
}

self.addEventListener('message', (event: ExtendableMessageEvent) => {
  if (!isTrustedOrigin(event.origin)) return;
  const data = event.data;
  if (data.type === 'skip-waiting') {
    self.skipWaiting();
  } else if (data.type === 'clear-cache') {
    caches.keys().then((names) => {
      names.forEach((name) => caches.delete(name));
    });
  } else if (data.type === 'prefetch-urls') {
    const urls: string[] = data.urls;
    urls.forEach((url) => {
      fetch(url);
    });
  }
});

export async function handleCacheOperation(event: ExtendableMessageEvent): Promise<void> {
  if (!isTrustedOrigin(event.origin)) return;
  if (event.data.action === 'flush') {
    const keys = await caches.keys();
    await Promise.all(keys.map((k) => caches.delete(k)));
  }
}
