// [frensense]
// observation: The service worker listens for postMessage events but does not verify the origin of the message sender. Any script running on the same origin (or an XSS victim) can send arbitrary messages that the service worker processes as trusted commands.
// impact: An attacker who achieves XSS on the page can send a postMessage to the service worker, triggering sensitive operations like cache deletion, request interception, or credential exfiltration without origin validation.
// improvement: Always verify event.origin in the service worker's message handler before processing commands.

self.addEventListener('message', (event: ExtendableMessageEvent) => {
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
  if (event.data.action === 'flush') {
    const keys = await caches.keys();
    await Promise.all(keys.map((k) => caches.delete(k)));
  }
}
