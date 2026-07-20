// SAFE: Null check ensures the property access is safe

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function getUserEmail(userId: string) {
  const user = await prisma.user.findUnique({ where: { id: userId } });
  if (!user) throw new Error('User not found');
  return user.email;
}
