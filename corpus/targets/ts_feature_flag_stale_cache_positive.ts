// [frensense]
// observation: Feature flag values are cached for a hardcoded long duration (e.g., 1 hour) without a cache-busting mechanism when flags are updated in the admin panel.
// impact: Users can access newly disabled premium features for up to an hour after revocation, or newly enabled features are delayed for paying customers.
// improvement: Use short TTLs for feature flags, or implement a publish/subscribe invalidation mechanism when flags change.

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

  await redis.setex(cacheKey, 3600, enabled.toString());

  return enabled;
}
