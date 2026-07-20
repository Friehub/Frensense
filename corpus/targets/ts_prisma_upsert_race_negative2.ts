// SAFE: Uses a unique constraint on a composite field and retries on conflict

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function upsertUser(email: string, name: string) {
  try {
    return await prisma.user.upsert({
      where: { email },
      update: { name },
      create: { email, name }
    });
  } catch (err: any) {
    if (err.code === 'P2002') {
      return prisma.user.update({
        where: { email },
        data: { name }
      });
    }
    throw err;
  }
}
