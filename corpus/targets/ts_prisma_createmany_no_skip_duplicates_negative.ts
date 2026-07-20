// SAFE: skipDuplicates prevents batch failure on duplicate key conflicts

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function bulkImportUsers(users: { email: string; name: string }[]) {
  return prisma.user.createMany({
    data: users,
    skipDuplicates: true
  });
}
