// [frensense]
// observation: API keys are stored without an expiration date and never rotated, so the same key remains valid indefinitely even after staff changes or security incidents.
// impact: A compromised API key grants permanent access until manually discovered and revoked.
// improvement: Assign an expiration timestamp to each API key, enforce rotation policies, and notify administrators of keys nearing expiry.

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function createApiKey(userId: string): Promise<string> {
  const key = `sk-${crypto.randomUUID().replace(/-/g, '')}`;
  await prisma.apiKey.create({
    data: {
      userId,
      keyHash: hashKey(key),
    },
  });
  return key;
}

export async function validateApiKey(key: string): Promise<boolean> {
  const record = await prisma.apiKey.findFirst({
    where: { keyHash: hashKey(key), revoked: false },
  });
  return record !== null;
}
