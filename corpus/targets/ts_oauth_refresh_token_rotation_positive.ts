// [frensense]
// observation: Refresh token is never rotated after use, allowing the same token to be reused indefinitely.
// impact: A leaked refresh token remains valid forever, giving an attacker persistent access without detection.
// improvement: Rotate the refresh token on each use by issuing a new refresh token and invalidating the old one.

import { Injectable } from '@nestjs/common';
import { JwtService } from '@nestjs/jwt';

@Injectable()
export class TokenService {
  constructor(private jwt: JwtService) {}

  async refreshToken(oldRefreshToken: string) {
    const payload = this.jwt.verify(oldRefreshToken);
    return {
      access_token: this.jwt.sign({ sub: payload.sub }),
      refresh_token: oldRefreshToken,
    };
  }
}
