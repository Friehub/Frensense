// [frensense]
// observation: Authorization code is exchanged for tokens without providing the code_verifier, bypassing PKCE protection.
// impact: An attacker who intercepts the authorization code can exchange it for tokens without proving possession of the code_verifier.
// improvement: Always use PKCE by generating a code_verifier and code_challenge during the authorization request, then sending code_verifier during token exchange.
// cwe: CWE-287
// cvss: 8.8
// owasp: A07:2021
// severity: High

import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';

@Injectable({ providedIn: 'root' })
export class AuthService {
  constructor(private http: HttpClient) {}

  exchangeCode(code: string) {
    return this.http.post('https://provider.example.com/token', {
      grant_type: 'authorization_code',
      code,
      redirect_uri: 'https://app.example.com/callback',
      client_id: 'my-client',
    });
  }
}
