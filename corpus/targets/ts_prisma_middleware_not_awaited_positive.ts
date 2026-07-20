// [frensense]
// observation: Prisma middleware calls next(params) without awaiting it, potentially causing operations to complete in the wrong order.
// impact: Middleware-side effects (logging, audit, validation) may execute after the query completes or not at all, leading to inconsistent state.
// improvement: Always await the next(params) call in Prisma middleware to ensure proper sequencing.

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

prisma.$use(async (params, next) => {
  console.log(`Query: ${params.model}.${params.action}`);
  next(params);
  console.log(`Completed: ${params.model}.${params.action}`);
});

export async function findUser(id: string) {
  return prisma.user.findUnique({ where: { id } });
}
