// SAFE: Short TTL with Redis key prefix for bulk invalidation

import { Redis } from 'ioredis';
import { PrismaClient } from '@prisma/client';

const redis = new Redis();
const prisma = new PrismaClient();

export async function isFeatureEnabled(userId: string, feature: string): Promise<boolean> {
  const cacheKey = `feature_flag:${feature}`;
  const cached = await redis.get(cacheKey);

  if (cached !== null) {
    return cached === 'true';
  }

  const flag = await prisma.featureFlag.findUnique({ where: { name: feature } });
  const enabled = flag?.enabled ?? false;

  await redis.setex(cacheKey, 60, enabled.toString());

  return enabled;
}

export async function invalidateFeatureCache(feature: string) {
  await redis.del(`feature_flag:${feature}`);
}
