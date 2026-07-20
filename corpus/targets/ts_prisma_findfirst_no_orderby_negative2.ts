// SAFE: Alternative using explicit sort field for deterministic ordering

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function getOldestSubscription(userId: string) {
  return prisma.subscription.findFirst({
    where: { userId, status: 'active' },
    orderBy: { id: 'asc' }
  });
}
