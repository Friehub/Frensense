// SAFE: PKCE enforced in a wrapper client

import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { firstValueFrom } from 'rxjs';

@Injectable({ providedIn: 'root' })
export class PkceAuthService {
  constructor(private http: HttpClient) {}

  async exchangeCode(code: string, codeVerifier?: string) {
    if (!codeVerifier) {
      throw new Error('PKCE code_verifier is required for token exchange');
    }
    return firstValueFrom(
      this.http.post('https://provider.example.com/token', {
        grant_type: 'authorization_code',
        code,
        code_verifier: codeVerifier,
        redirect_uri: 'https://app.example.com/callback',
        client_id: 'my-client',
      })
    );
  }
}
