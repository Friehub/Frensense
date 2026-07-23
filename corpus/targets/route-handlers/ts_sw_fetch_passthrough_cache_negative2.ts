// SAFE alternative: cache-on-request pattern for static assets and use network-only for API calls
const STATIC_CACHE = 'static-v2';
const API_PREFIX = '/api/';

function isApiRequest(url: string): boolean {
  return url.includes(API_PREFIX);
}

self.addEventListener('fetch', (event: FetchEvent) => {
  if (isApiRequest(event.request.url)) {
    event.respondWith(fetch(event.request));
    return;
  }
  event.respondWith(
    caches.open(STATIC_CACHE).then((cache) => {
      return cache.match(event.request).then((cached) => {
        const fetchPromise = fetch(event.request).then((response) => {
          cache.put(event.request, response.clone());
          return response;
        });
        return cached || fetchPromise;
      });
    }),
  );
});
