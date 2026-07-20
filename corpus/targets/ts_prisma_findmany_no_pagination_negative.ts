// SAFE: findMany() uses a take limit to prevent unbounded results

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

const PAGE_SIZE = 50;

export async function getUsersPage(page: number = 1) {
  return prisma.user.findMany({
    take: PAGE_SIZE,
    skip: (page - 1) * PAGE_SIZE
  });
}
