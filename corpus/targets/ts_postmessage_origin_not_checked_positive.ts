// [frensense]
// observation: The window.addEventListener('message', ...) handler processes incoming postMessage data without checking event.origin. Any cross-origin iframe or popup window can send messages that are treated as trusted.
// impact: An attacker embeds the vulnerable page in an iframe or opens a popup and sends a crafted postMessage. The handler processes the malicious data, potentially performing unauthorized actions, revealing sensitive data, or executing scripts.
// improvement: Always validate event.origin against a whitelist of trusted origins before processing the message data.

export function setupMessageListener(): void {
  window.addEventListener('message', (event: MessageEvent) => {
    const data = event.data;
    if (data.action === 'navigate') {
      window.location.href = data.url;
    } else if (data.action === 'execute') {
      eval(data.code);
    }
  });
}

export function initWidgetCommunication(): void {
  window.addEventListener('message', (event: MessageEvent) => {
    const container = document.getElementById('widget-container');
    if (container) {
      container.innerHTML = event.data.html;
    }
  });
}
