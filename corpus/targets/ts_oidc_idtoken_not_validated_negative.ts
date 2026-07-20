// SAFE: ID token verified with JWKS, iss, aud, and exp validation

import { Injectable } from '@angular/core';
import jose from 'jose';

@Injectable({ providedIn: 'root' })
export class OidcService {
  private readonly expectedIssuer = 'https://provider.example.com';
  private readonly expectedAudience = 'my-client';
  private jwks: jose.JWKS | null = null;

  private async getJWKS(): Promise<jose.JWKS> {
    if (!this.jwks) {
      const response = await fetch('https://provider.example.com/.well-known/jwks.json');
      this.jwks = await response.json();
    }
    return this.jwks;
  }

  async handleIdToken(idToken: string) {
    const jwks = await this.getJWKS();
    const { payload } = await jose.jwtVerify(idToken, jwks, {
      issuer: this.expectedIssuer,
      audience: this.expectedAudience,
    });
    return payload;
  }
}
