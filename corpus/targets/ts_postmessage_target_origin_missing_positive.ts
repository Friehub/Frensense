// [frensense]
// observation: The code calls window.postMessage with '*' as the target origin, allowing any cross-origin window to receive the message. Sensitive data sent via postMessage may be intercepted by a malicious iframe or popup.
// impact: An attacker's cross-origin iframe or popup receives the message and can exfiltrate sensitive data such as authentication tokens, user details, or internal state information.
// improvement: Specify the exact target origin (not '*') when calling postMessage. If the target window is an iframe, use its origin explicitly.

export function sendAuthToken(token: string): void {
  const iframe = document.getElementById('auth-iframe') as HTMLIFrameElement;
  if (iframe && iframe.contentWindow) {
    iframe.contentWindow.postMessage({ type: 'auth', token }, '*');
  }
}

export function broadcastUserData(userData: object): void {
  window.opener.postMessage({ type: 'user-data', payload: userData }, '*');
}
