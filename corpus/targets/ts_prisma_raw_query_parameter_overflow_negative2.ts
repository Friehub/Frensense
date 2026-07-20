// SAFE: Use Prisma's built-in findMany with IN instead of raw query

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function getUsersByIds(ids: number[]) {
  return prisma.user.findMany({
    where: { id: { in: ids } },
  });
}
