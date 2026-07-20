// [frensense]
// observation: Prisma's findMany() is called without a take limit, potentially returning millions of records.
// impact: An unbounded query can exhaust database connections, consume excessive memory, and lead to denial of service.
// improvement: Always set a take limit on findMany() calls, or use pagination with cursor-based queries.

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function getAllUsers() {
  return prisma.user.findMany();
}
