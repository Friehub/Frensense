// SAFE: Rotated via Prisma with family tracking

import { Injectable } from '@nestjs/common';
import { JwtService } from '@nestjs/jwt';
import { PrismaService } from '../prisma/prisma.service';
import { v4 as uuidv4 } from 'uuid';

@Injectable()
export class TokenService {
  constructor(
    private jwt: JwtService,
    private prisma: PrismaService,
  ) {}

  async refreshToken(oldRefreshToken: string, userId: string) {
    const tokenRecord = await this.prisma.refreshToken.findUnique({
      where: { token: oldRefreshToken },
    });

    if (!tokenRecord || tokenRecord.revokedAt) {
      throw new Error('Invalid or revoked refresh token');
    }

    await this.prisma.refreshToken.update({
      where: { id: tokenRecord.id },
      data: { revokedAt: new Date() },
    });

    const newToken = uuidv4();
    await this.prisma.refreshToken.create({
      data: {
        token: newToken,
        userId,
        family: tokenRecord.family,
        expiresAt: new Date(Date.now() + 30 * 24 * 60 * 60 * 1000),
      },
    });

    return {
      access_token: this.jwt.sign({ sub: userId }),
      refresh_token: newToken,
    };
  }
}
