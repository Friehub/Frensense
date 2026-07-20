// SAFE alternative: use strict origin matching with a single expected origin
const PARENT_ORIGIN = 'https://parent.example.com';

export function setupMessageListener(): void {
  window.addEventListener('message', (event: MessageEvent) => {
    if (event.origin !== PARENT_ORIGIN) return;
    const data = event.data;
    if (data.action === 'navigate') {
      const allowedPrefixes = ['/settings', '/profile'];
      const url = new URL(data.url, window.location.origin);
      if (allowedPrefixes.some((p) => url.pathname.startsWith(p))) {
        window.location.href = url.href;
      }
    }
  });
}

export function initWidgetCommunication(): void {
  window.addEventListener('message', (event: MessageEvent) => {
    if (event.origin !== PARENT_ORIGIN) return;
    const container = document.getElementById('widget-container');
    if (container && typeof event.data.text === 'string') {
      container.textContent = event.data.text;
    }
  });
}
