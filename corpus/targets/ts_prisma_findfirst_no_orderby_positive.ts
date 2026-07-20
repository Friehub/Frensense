// [frensense]
// observation: findFirst is called without orderBy, so the database may return an unpredictable row when multiple rows match the where clause.
// impact: Non-deterministic behavior can lead to logical bugs, incorrect state transitions, or data leakage when the "first" row varies across calls.
// improvement: Always specify an orderBy clause with findFirst to guarantee which matching row is returned.

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function getOldestSubscription(userId: string) {
  return prisma.subscription.findFirst({
    where: { userId, status: 'active' }
  });
}
