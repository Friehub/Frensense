// [frensense]
// observation: OAuth consent screen is always skipped with auto-approval, authorizing all requested scopes without user interaction.
// impact: Users are not informed about which permissions are being granted, potentially authorizing excessive scopes without awareness.
// improvement: Always show the consent screen on first authorization per scope combination, or require explicit user consent for sensitive scopes.
// cwe: CWE-287
// cvss: 8.8
// owasp: A07:2021
// severity: High

import { Injectable } from '@nestjs/common';

@Injectable()
export class OAuthService {
  authorize(clientId: string, userId: string, scopes: string[]) {
    return {
      redirectUri: `https://app.example.com/callback?code=${this.generateCode()}`,
    };
  }

  private generateCode(): string {
    return Math.random().toString(36).substring(2);
  }
}
