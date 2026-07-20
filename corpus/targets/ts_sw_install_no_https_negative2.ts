// SAFE alternative: redirect HTTP to HTTPS before registration and fallback gracefully
export function registerServiceWorker(): void {
  if (location.protocol !== 'https:' && location.hostname !== 'localhost') {
    location.replace(`https://${location.host}${location.pathname}`);
    return;
  }
  if ('serviceWorker' in navigator) {
    navigator.serviceWorker.register('/sw.js', {
      scope: '/',
      updateViaCache: 'none',
    });
  }
}

export function registerAnalyticsWorker(): void {
  if (location.protocol !== 'https:' && location.hostname !== 'localhost') {
    return;
  }
  if ('serviceWorker' in navigator) {
    navigator.serviceWorker.register('/analytics-worker.js', { scope: '/analytics' });
  }
}
