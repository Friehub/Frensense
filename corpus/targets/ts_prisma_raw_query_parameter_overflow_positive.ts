// [frensense]
// observation: A raw query interpolates a large array of User IDs directly into the SQL, causing parameter overflow and database crashes for large inputs.
// impact: Passing thousands of IDs crashes the database with 'too many parameters' or generates an enormous query that exhausts server memory.
// improvement: Use chunking to split large IN clauses into batches, or use a temporary table to hold the ID list.

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function getUsersByIds(ids: number[]) {
  return prisma.$queryRawUnsafe(
    `SELECT * FROM "User" WHERE id IN (${ids.join(',')})`
  );
}
