// SAFE: Authorization code flow with PKCE used instead of implicit grant

import { Injectable } from '@angular/core';
import { Router } from '@angular/router';

@Injectable({ providedIn: 'root' })
export class PkceAuthService {
  constructor(private router: Router) {}

  async login() {
    const codeVerifier = this.generateCodeVerifier();
    const codeChallenge = await this.generateCodeChallenge(codeVerifier);
    sessionStorage.setItem('code_verifier', codeVerifier);

    const params = new URLSearchParams({
      response_type: 'code',
      client_id: 'my-client',
      redirect_uri: 'https://app.example.com/callback',
      scope: 'openid profile',
      code_challenge: codeChallenge,
      code_challenge_method: 'S256',
    });
    window.location.href = `https://provider.example.com/authorize?${params}`;
  }

  private generateCodeVerifier(): string {
    const arr = new Uint8Array(32);
    crypto.getRandomValues(arr);
    return btoa(String.fromCharCode(...arr)).replace(/[^a-zA-Z0-9\-._~]/g, '');
  }

  private async generateCodeChallenge(verifier: string): Promise<string> {
    const hash = await crypto.subtle.digest('SHA-256', new TextEncoder().encode(verifier));
    return btoa(String.fromCharCode(...new Uint8Array(hash))).replace(/\+/g, '-').replace(/\//g, '_').replace(/=+$/, '');
  }
}
