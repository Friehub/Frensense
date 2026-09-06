// SAFE alternative: crypto.randomBytes for raw entropy
import { randomBytes } from 'node:crypto';

function generateCsrfToken(): string {
  return randomBytes(32).toString('hex');
}

function createPasswordResetToken(userId: string): string {
  return `${userId}_${randomBytes(24).toString('base64url')}`;
}
