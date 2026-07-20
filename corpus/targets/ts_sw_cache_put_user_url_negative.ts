// SAFE: Validate that the URL matches the app's origin before caching.

const APP_ORIGIN = 'https://app.example.com';

self.addEventListener('fetch', (event: FetchEvent) => {
  const url = new URL(event.request.url);
  const userUrlStr = url.searchParams.get('cachedUrl');
  if (!userUrlStr) return;

  const userUrl = new URL(userUrlStr);
  if (userUrl.origin !== APP_ORIGIN) {
    console.error('Blocked cross-origin cache write:', userUrlStr);
    return;
  }

  event.respondWith(
    caches.open('dynamic-v1').then((cache) => {
      return fetch(userUrlStr).then((response) => {
        cache.put(userUrlStr, response.clone());
        return response;
      });
    })
  );
});
