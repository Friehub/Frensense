// SAFE: check for secure context before registering the service worker
export function registerServiceWorker(): void {
  if ('serviceWorker' in navigator && window.isSecureContext) {
    navigator.serviceWorker.register('/sw.js', {
      scope: '/',
    });
  }
}

export function registerAnalyticsWorker(): void {
  if ('serviceWorker' in navigator && window.isSecureContext) {
    navigator.serviceWorker.register('/analytics-worker.js', { scope: '/analytics' });
  }
}
