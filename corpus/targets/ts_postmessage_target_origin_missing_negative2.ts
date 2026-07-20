// SAFE alternative: use a dynamic origin check via the source window's location
export function sendAuthToken(token: string): void {
  const iframe = document.getElementById('auth-iframe') as HTMLIFrameElement;
  if (iframe && iframe.contentWindow) {
    try {
      const iframeOrigin = iframe.contentWindow.location.origin;
      if (iframeOrigin === 'https://auth.example.com') {
        iframe.contentWindow.postMessage({ type: 'auth', token }, iframeOrigin);
      }
    } catch {
      if (iframe.src.startsWith('https://auth.example.com')) {
        iframe.contentWindow.postMessage({ type: 'auth', token }, 'https://auth.example.com');
      }
    }
  }
}

export function broadcastUserData(userData: object): void {
  if (window.opener && window.opener.location.origin === 'https://parent.example.com') {
    window.opener.postMessage({ type: 'user-data', payload: userData }, 'https://parent.example.com');
  }
}
