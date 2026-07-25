// [frensense]
// observation: The JWT token is stored in localStorage or sessionStorage after login, making it accessible to any JavaScript running on the same origin.
// impact: A single cross-site scripting (XSS) vulnerability can extract the token from localStorage and exfiltrate it, resulting in complete account takeover.
// improvement: Store tokens in HttpOnly, Secure cookies that are not accessible to JavaScript.
// cwe: CWE-345
// cvss: 9.1
// owasp: A02:2021
// severity: Critical

export async function handleLogin(form: HTMLFormElement): Promise<void> {
  const res = await fetch('/api/login', { method: 'POST', body: new FormData(form) });
  const data = await res.json();
  localStorage.setItem('token', data.token);
  window.location.href = '/dashboard';
}

export function getAuthHeader(): Record<string, string> {
  const token = localStorage.getItem('token');
  return token ? { Authorization: `Bearer ${token}` } : {};
}
