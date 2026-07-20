// SAFE: specify the exact target origin in postMessage
const AUTH_IFRAME_ORIGIN = 'https://auth.example.com';

export function sendAuthToken(token: string): void {
  const iframe = document.getElementById('auth-iframe') as HTMLIFrameElement;
  if (iframe && iframe.contentWindow) {
    iframe.contentWindow.postMessage({ type: 'auth', token }, AUTH_IFRAME_ORIGIN);
  }
}

export function broadcastUserData(userData: object): void {
  const parentOrigin = document.referrer ? new URL(document.referrer).origin : '';
  if (parentOrigin === 'https://parent.example.com' && window.opener) {
    window.opener.postMessage({ type: 'user-data', payload: userData }, parentOrigin);
  }
}
