// SAFE: findUniqueOrThrow guarantees a non-null result or throws

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function getUserEmail(userId: string) {
  const user = await prisma.user.findUniqueOrThrow({ where: { id: userId } });
  return user.email;
}
