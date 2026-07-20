// SAFE: Use select to explicitly whitelist only safe fields

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function getUserProfile(userId: string) {
  return prisma.user.findUnique({
    where: { id: userId },
    select: {
      id: true,
      name: true,
      email: true,
      posts: { select: { title: true, body: true } }
    }
  });
}
