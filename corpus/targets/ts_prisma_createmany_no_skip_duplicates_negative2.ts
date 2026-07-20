// SAFE: Individual creates with error isolation per record

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function bulkImportUsers(users: { email: string; name: string }[]) {
  const results: Array<{ email: string; name: string } | null> = [];
  for (const user of users) {
    try {
      results.push(await prisma.user.create({ data: user }));
    } catch {
      results.push(null);
    }
  }
  return results;
}
