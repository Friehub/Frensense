// SAFE: crypto.randomUUID() for all security tokens
import { randomUUID, randomBytes } from 'node:crypto';

function generateCsrfToken(): string {
  return randomUUID();
}

async function createPasswordResetToken(userId: string): Promise<string> {
  const token = randomUUID().replace(/-/g, '');
  const buf = randomBytes(16).toString('hex');
  return `${userId}_${buf}`;
}

function generateNonce(): string {
  return randomUUID();
}
