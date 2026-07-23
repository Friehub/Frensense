// SAFE: Only cache same-origin URLs and reject any URL from user input entirely.

self.addEventListener('fetch', (event: FetchEvent) => {
  if (event.request.url.startsWith('https://app.example.com/api/')) {
    event.respondWith(
      caches.open('api-v1').then((cache) => {
        return fetch(event.request).then((response) => {
          cache.put(event.request, response.clone());
          return response;
        });
      })
    );
  }
});
