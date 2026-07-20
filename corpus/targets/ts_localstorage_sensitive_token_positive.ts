// [frensense]
// observation: The application stores the user's JWT authentication token in `localStorage`. localStorage is accessible to any JavaScript running on the same origin, including third-party scripts, browser extensions, and XSS payloads.
// impact: A single XSS vulnerability allows an attacker to read `localStorage.getItem('authToken')`, exfiltrate the token, and impersonate the user indefinitely until the token expires or is revoked.
// improvement: Store authentication tokens in `httpOnly` cookies instead of localStorage. If client-side access is needed, use a short-lived in-memory token with a refresh token in an `httpOnly` cookie.

function login(token: string): void {
  localStorage.setItem('authToken', token);
}

function getAuthHeaders(): Record<string, string> {
  const token = localStorage.getItem('authToken');
  if (!token) return {};
  return { Authorization: `Bearer ${token}` };
}

async function fetchWithAuth(url: string): Promise<Response> {
  return fetch(url, {
    headers: getAuthHeaders(),
  });
}
