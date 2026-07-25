// [frensense]
// observation: OAuth2 implicit grant flow is used, returning the access token in the URL fragment after redirect.
// impact: Access tokens are exposed in the browser URL and browser history, making them vulnerable to leakage via referrer headers and shoulder surfing.
// improvement: Use the authorization code flow with PKCE instead of the deprecated implicit grant.
// cwe: CWE-287
// cvss: 8.8
// owasp: A07:2021
// severity: High

import { Injectable } from '@angular/core';
import { Router } from '@angular/router';

@Injectable({ providedIn: 'root' })
export class ImplicitAuthService {
  constructor(private router: Router) {}

  login() {
    const params = new URLSearchParams({
      response_type: 'token',
      client_id: 'my-client',
      redirect_uri: 'https://app.example.com/callback',
      scope: 'openid profile',
    });
    window.location.href = `https://provider.example.com/authorize?${params}`;
  }

  handleCallback() {
    const hash = window.location.hash.substring(1);
    const params = new URLSearchParams(hash);
    const token = params.get('access_token');
    if (token) {
      localStorage.setItem('access_token', token);
      this.router.navigate(['/dashboard']);
    }
  }
}
