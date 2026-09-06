// SAFE: API keys include an expiry date and rotation is enforced at validation time
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();
const KEY_MAX_AGE_DAYS = 90;

export async function createApiKey(userId: string): Promise<string> {
  const key = `sk-${crypto.randomUUID().replace(/-/g, '')}`;
  await prisma.apiKey.create({
    data: {
      userId,
      keyHash: hashKey(key),
      expiresAt: new Date(Date.now() + KEY_MAX_AGE_DAYS * 86400000),
    },
  });
  return key;
}

export async function validateApiKey(key: string): Promise<boolean> {
  const record = await prisma.apiKey.findFirst({
    where: {
      keyHash: hashKey(key),
      revoked: false,
      expiresAt: { gt: new Date() },
    },
  });
  return record !== null;
}

function hashKey(key: string): string {
  return crypto.createHash('sha256').update(key).digest('hex');
}
