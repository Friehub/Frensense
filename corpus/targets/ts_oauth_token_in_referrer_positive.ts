// [frensense]
// observation: The OAuth implicit flow places the access token in the URL fragment after the redirect, and the application reads it via JavaScript. The URL (with fragment) is sent as the Referer header to external resources.
// impact: If the page loads external resources (images, fonts, analytics), the token leaks via the Referer header to third-party servers.
// improvement: Use the authorization code flow with PKCE instead of implicit flow, or use form_post response mode.

export function handleRedirect(): void {
  const params = new URLSearchParams(window.location.hash.substring(1));
  const token = params.get('access_token');
  if (token) {
    localStorage.setItem('token', token);
    window.location.href = '/dashboard';
  }
}
