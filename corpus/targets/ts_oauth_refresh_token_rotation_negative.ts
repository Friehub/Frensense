// SAFE: Refresh token rotated on each use, old one invalidated

import { Injectable } from '@nestjs/common';
import { JwtService } from '@nestjs/jwt';
import { v4 as uuidv4 } from 'uuid';

@Injectable()
export class TokenService {
  private readonly usedTokens = new Set<string>();

  constructor(private jwt: JwtService) {}

  async refreshToken(oldRefreshToken: string) {
    if (this.usedTokens.has(oldRefreshToken)) {
      throw new Error('Refresh token reuse detected');
    }
    this.usedTokens.add(oldRefreshToken);

    const payload = this.jwt.verify(oldRefreshToken);
    const newRefreshToken = uuidv4();

    return {
      access_token: this.jwt.sign({ sub: payload.sub }),
      refresh_token: newRefreshToken,
    };
  }
}
