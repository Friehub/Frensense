// SAFE: orderBy ensures the intended row is always returned first

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function getOldestSubscription(userId: string) {
  return prisma.subscription.findFirst({
    where: { userId, status: 'active' },
    orderBy: { createdAt: 'asc' }
  });
}
