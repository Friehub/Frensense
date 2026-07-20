// SAFE: Embedded JWT expiry in the reset token itself (self-contained)
import jwt from 'jsonwebtoken';

const RESET_SECRET = process.env.RESET_SECRET!;

export function generateResetToken(email: string): string {
  return jwt.sign({ email, purpose: 'password-reset' }, RESET_SECRET, { expiresIn: '15m' });
}

export function verifyResetToken(token: string): { email: string } | null {
  try {
    const payload = jwt.verify(token, RESET_SECRET, { algorithms: ['HS256'] }) as any;
    if (payload.purpose !== 'password-reset') return null;
    return { email: payload.email };
  } catch { return null; }
}
