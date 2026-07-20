// SAFE: Explicit date filter ensures only stale records are affected

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function deactivateExpiredUsers() {
  const cutoff = new Date();
  cutoff.setDate(cutoff.getDate() - 90);
  return prisma.user.updateMany({
    where: { lastLogin: { lt: cutoff } },
    data: { active: false }
  });
}
