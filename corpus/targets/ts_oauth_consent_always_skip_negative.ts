// SAFE: Consent screen shown, user must approve scopes

import { Injectable } from '@nestjs/common';
import { PrismaService } from '../prisma/prisma.service';
import { randomBytes } from 'node:crypto';

@Injectable()
export class OAuthService {
  constructor(private prisma: PrismaService) {}

  async authorize(clientId: string, userId: string, scopes: string[]) {
    const existingConsent = await this.prisma.oauthConsent.findUnique({
      where: { userId_clientId: { userId, clientId } },
    });

    if (!existingConsent || existingConsent.scopes.join(',') !== scopes.join(',')) {
      return { requiresConsent: true, scopes, clientId };
    }

    return {
      requiresConsent: false,
      redirectUri: `https://app.example.com/callback?code=${this.generateCode()}`,
    };
  }

  async confirmConsent(clientId: string, userId: string, scopes: string[]) {
    await this.prisma.oauthConsent.upsert({
      where: { userId_clientId: { userId, clientId } },
      update: { scopes },
      create: { userId, clientId, scopes },
    });

    return {
      redirectUri: `https://app.example.com/callback?code=${this.generateCode()}`,
    };
  }

  private generateCode(): string {
    return randomBytes(16).toString('hex');
  }
}
