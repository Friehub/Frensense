// SAFE: validate event.origin before processing the message
const TRUSTED_ORIGINS = new Set(['https://app.example.com', 'https://widget.example.com']);

export function setupMessageListener(): void {
  window.addEventListener('message', (event: MessageEvent) => {
    if (!TRUSTED_ORIGINS.has(event.origin)) return;
    const data = event.data;
    if (data.action === 'navigate') {
      window.location.href = data.url;
    } else if (data.action === 'execute') {
      console.error('execute action not allowed');
    }
  });
}

export function initWidgetCommunication(): void {
  window.addEventListener('message', (event: MessageEvent) => {
    if (!TRUSTED_ORIGINS.has(event.origin)) return;
    const container = document.getElementById('widget-container');
    if (container) {
      container.textContent = event.data.text || '';
    }
  });
}
