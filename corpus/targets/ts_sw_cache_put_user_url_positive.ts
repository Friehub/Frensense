// [frensense]
// observation: The service worker's fetch handler stores the user-supplied URL as a cache key via `cache.put(userUrl, response)`. An attacker can inject arbitrary URLs (e.g., `https://attacker.com/evil.js`) that the cache will store and serve.
// impact: An attacker can poison the service worker cache with malicious content from arbitrary origins. When the app later serves cached content, users receive attacker-controlled scripts, enabling XSS at the app's origin.
// improvement: Validate that the cached URL matches the same origin as the application before storing.

self.addEventListener('fetch', (event: FetchEvent) => {
  const url = new URL(event.request.url);
  const userUrl = url.searchParams.get('cachedUrl');
  if (!userUrl) return;

  event.respondWith(
    caches.open('dynamic-v1').then((cache) => {
      return fetch(userUrl).then((response) => {
        cache.put(userUrl, response.clone());
        return response;
      });
    })
  );
});
