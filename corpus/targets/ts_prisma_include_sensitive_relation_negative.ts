// SAFE: Sensitive relation excluded from the include

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function getUserProfile(userId: string) {
  return prisma.user.findUnique({
    where: { id: userId },
    include: { posts: true }
  });
}
