// [frensense]
// observation: createMany is called without skipDuplicates: true, so a single conflicting record causes the entire batch to abort.
// impact: A duplicate entry in a bulk import can fail the whole operation, losing all other valid records and causing data inconsistency.
// improvement: Add skipDuplicates: true to let non-conflicting rows succeed, or handle duplicates individually.

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function bulkImportUsers(users: { email: string; name: string }[]) {
  return prisma.user.createMany({
    data: users
  });
}
