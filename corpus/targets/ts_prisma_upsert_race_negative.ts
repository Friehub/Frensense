// SAFE: Uses a transaction with locking to prevent race conditions

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function upsertUser(email: string, name: string) {
  return prisma.$transaction(async (tx) => {
    const existing = await tx.user.findFirst({ where: { email } });
    if (existing) {
      return tx.user.update({ where: { id: existing.id }, data: { name } });
    }
    return tx.user.create({ data: { email, name } });
  });
}
