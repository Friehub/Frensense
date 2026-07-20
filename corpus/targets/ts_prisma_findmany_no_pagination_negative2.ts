// SAFE: Uses cursor-based pagination for efficient large dataset traversal

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

const PAGE_SIZE = 50;

export async function getUsersCursor(cursor?: string) {
  return prisma.user.findMany({
    take: PAGE_SIZE,
    ...(cursor ? { skip: 1, cursor: { id: cursor } } : {}),
    orderBy: { id: 'asc' }
  });
}
