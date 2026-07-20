// SAFE: Always require consent for sensitive scopes

import { Injectable } from '@nestjs/common';
import { randomBytes } from 'crypto';

const SENSITIVE_SCOPES = ['email', 'profile', 'offline_access'];

@Injectable()
export class OAuthService {
  authorize(clientId: string, userId: string, scopes: string[]) {
    const hasSensitive = scopes.some(s => SENSITIVE_SCOPES.includes(s));
    if (hasSensitive) {
      return { requiresConsent: true, scopes, clientId };
    }
    return {
      requiresConsent: false,
      redirectUri: `https://app.example.com/callback?code=${this.generateCode()}`,
    };
  }

  private generateCode(): string {
    return randomBytes(16).toString('hex');
  }
}
