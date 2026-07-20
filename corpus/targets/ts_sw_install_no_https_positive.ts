// [frensense]
// observation: A service worker is registered from an HTTP page without requiring HTTPS. Service workers can only intercept requests when served over secure context, but registering from the same origin over HTTP creates a vector for man-in-the-middle attacks to inject a malicious service worker.
// impact: An attacker on the same network can inject a malicious service worker via MITM, intercepting and modifying all network requests for the lifetime of the registered worker, including credentials and API responses.
// improvement: Ensure the page is served over HTTPS before registering service workers. Use the 'updateViaCache' option and implement integrity checks on the worker script.

export function registerServiceWorker(): void {
  if ('serviceWorker' in navigator) {
    navigator.serviceWorker.register('/sw.js', {
      scope: '/',
    });
  }
}

export function registerAnalyticsWorker(): void {
  if ('serviceWorker' in navigator) {
    navigator.serviceWorker.register('/analytics-worker.js', { scope: '/analytics' });
  }
}
