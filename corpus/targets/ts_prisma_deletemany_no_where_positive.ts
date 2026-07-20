// [frensense]
// observation: deleteMany or updateMany is called without a where clause, targeting every row in the table.
// impact: All records in the table are unintentionally deleted or modified, causing catastrophic data loss or corruption.
// improvement: Always include a where clause on deleteMany and updateMany, or gate these operations behind an explicit confirmation.

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function deactivateExpiredUsers() {
  return prisma.user.updateMany({
    data: { active: false }
  });
}
