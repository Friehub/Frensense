// SAFE: Uses Prisma's typed query with parameterized value instead of raw SQL

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function getUser(id: string) {
  const result = await prisma.user.findUnique({ where: { id } });
  return result;
}
