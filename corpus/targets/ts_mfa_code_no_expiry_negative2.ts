// SAFE: Uses TOTP (time-based) with 30-second window — codes inherently expire
import { authenticator } from 'otplib';

export function generateTotpSecret(): string {
  return authenticator.generateSecret();
}

export function verifyTotp(token: string, secret: string): boolean {
  return authenticator.verify({ token, secret });
}
