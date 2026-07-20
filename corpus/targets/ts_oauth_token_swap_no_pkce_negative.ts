// SAFE: PKCE code_verifier included in token exchange

import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';

@Injectable({ providedIn: 'root' })
export class AuthService {
  constructor(private http: HttpClient) {}

  exchangeCode(code: string, codeVerifier: string) {
    return this.http.post('https://provider.example.com/token', {
      grant_type: 'authorization_code',
      code,
      code_verifier: codeVerifier,
      redirect_uri: 'https://app.example.com/callback',
      client_id: 'my-client',
    });
  }
}
