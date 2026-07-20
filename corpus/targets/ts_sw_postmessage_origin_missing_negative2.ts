// SAFE alternative: validate the message came from a known client window
self.addEventListener('message', async (event: ExtendableMessageEvent) => {
  const client = await self.clients.get(event.source?.id || '');
  if (!client || client.url !== 'https://example.com/app') return;
  const data = event.data;
  if (data.type === 'skip-waiting') {
    self.skipWaiting();
  } else if (data.type === 'clear-cache') {
    caches.keys().then((names) => {
      names.forEach((name) => caches.delete(name));
    });
  }
});

export async function handleCacheOperation(event: ExtendableMessageEvent): Promise<void> {
  const client = await self.clients.get(event.source?.id || '');
  if (!client || !client.url.startsWith('https://example.com/')) return;
  if (event.data.action === 'flush') {
    const keys = await caches.keys();
    await Promise.all(keys.map((k) => caches.delete(k)));
  }
}
