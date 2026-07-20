// SAFE: next(params) awaited for correct sequencing

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

prisma.$use(async (params, next) => {
  console.log(`Query: ${params.model}.${params.action}`);
  const result = await next(params);
  console.log(`Completed: ${params.model}.${params.action}`);
  return result;
});

export async function findUser(id: string) {
  return prisma.user.findUnique({ where: { id } });
}
