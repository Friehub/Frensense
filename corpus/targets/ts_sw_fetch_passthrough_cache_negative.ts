// SAFE: only cache GET requests for static assets, not API calls
const CACHE_NAME = 'static-v1';
const STATIC_EXTENSIONS = /\.(js|css|png|jpg|svg|woff2?)$/;

function isStaticAsset(request: Request): boolean {
  if (request.method !== 'GET') return false;
  const url = new URL(request.url);
  return STATIC_EXTENSIONS.test(url.pathname);
}

self.addEventListener('fetch', (event: FetchEvent) => {
  if (!isStaticAsset(event.request)) {
    event.respondWith(fetch(event.request));
    return;
  }
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
