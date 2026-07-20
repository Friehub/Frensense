// SAFE: Where clause restricts the update to only the intended rows

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function deactivateExpiredUsers() {
  return prisma.user.updateMany({
    where: { expiresAt: { lt: new Date() } },
    data: { active: false }
  });
}
