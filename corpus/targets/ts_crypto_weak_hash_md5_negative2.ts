// SAFE alternative: argon2 for passwords, SHA-512 for integrity
import argon2 from 'argon2';

async function hashPassword(password: string): Promise<string> {
  return argon2.hash(password, { type: argon2.argon2id, memoryCost: 65536, timeCost: 3 });
}

function signPayload(payload: object, secret: string): string {
  const { createHash } = await import('node:crypto');
  return createHash('sha512').update(JSON.stringify({ payload, secret })).digest('hex');
}
