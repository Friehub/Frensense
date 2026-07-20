// SAFE: If raw SQL is required, uses Prisma's tagged template literal syntax

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function getUser(id: string) {
  const result = await prisma.$queryRaw`SELECT * FROM users WHERE id = ${id}`;
  return result;
}
