// [frensense]
// observation: The same state parameter value is reused across multiple OAuth authorization flows, making CSRF attacks possible.
// impact: An attacker can forge an authorization response using a captured state value, enabling CSRF-based account linking attacks.
// improvement: Generate a unique, cryptographically random state value for each authorization request and validate it on callback.
// cwe: CWE-287
// cvss: 8.8
// owasp: A07:2021
// severity: High

import { Injectable } from '@angular/core';

@Injectable({ providedIn: 'root' })
export class AuthService {
  private readonly state = 'fixed-state-value';

  getAuthorizationUrl(provider: string): string {
    const params = new URLSearchParams({
      response_type: 'code',
      client_id: 'my-client',
      redirect_uri: 'https://app.example.com/callback',
      state: this.state,
      scope: 'openid email',
    });
    return `https://${provider}.example.com/authorize?${params}`;
  }

  handleCallback(code: string, state: string): boolean {
    return state === this.state;
  }
}
