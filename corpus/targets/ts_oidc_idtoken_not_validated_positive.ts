// [frensense]
// observation: OIDC ID token is decoded without verifying its signature, issuer, or audience, accepting any forged token.
// impact: An attacker can craft a fake ID token with arbitrary claims, impersonating any user without valid credentials.
// improvement: Always verify the ID token's signature using the provider's JWKS, validate the iss and aud claims, and check the exp claim.
// cwe: CWE-287
// cvss: 8.8
// owasp: A07:2021
// severity: High

import { Injectable } from '@angular/core';
import { jwtDecode } from 'jwt-decode';

@Injectable({ providedIn: 'root' })
export class OidcService {
  handleIdToken(idToken: string) {
    const payload = jwtDecode(idToken);
    return {
      sub: (payload as any).sub,
      email: (payload as any).email,
      name: (payload as any).name,
    };
  }
}
