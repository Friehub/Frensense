// SAFE: next(params) awaited with error handling

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

prisma.$use(async (params, next) => {
  const start = Date.now();
  try {
    const result = await next(params);
    const duration = Date.now() - start;
    console.log(`${params.model}.${params.action} took ${duration}ms`);
    return result;
  } catch (error) {
    console.error(`${params.model}.${params.action} failed:`, error);
    throw error;
  }
});

export async function findUser(id: string) {
  return prisma.user.findUnique({ where: { id } });
}
