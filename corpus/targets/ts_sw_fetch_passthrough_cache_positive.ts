// [frensense]
// observation: The service worker's fetch handler caches all responses including sensitive API data without inspecting the request URL. Sensitive responses containing authentication tokens or personal data are stored in the Cache API and may be accessible from other contexts.
// impact: An XSS vulnerability or compromised third-party script can read sensitive cached data from the Cache API, including API responses with PII, tokens, or financial data.
// improvement: Only cache responses for static assets, never for authenticated API endpoints. Add request URL filtering to the fetch handler.

const CACHE_NAME = 'app-cache-v1';

self.addEventListener('fetch', (event: FetchEvent) => {
  event.respondWith(
    caches.match(event.request).then((cached) => {
      if (cached) return cached;
      return fetch(event.request).then((response) => {
        const clone = response.clone();
        caches.open(CACHE_NAME).then((cache) => {
          cache.put(event.request, clone);
        });
        return response;
      });
    }),
  );
});

self.addEventListener('activate', (event: ExtendableEvent) => {
  event.waitUntil(caches.open(CACHE_NAME));
});
