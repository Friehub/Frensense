// [frensense]
// observation: connectOrCreate uses user-controlled input in the where clause, so when the where condition does not match an existing record a new one is silently created instead.
// impact: Attackers can pollute the database with arbitrary records, bypass access control checks that rely on record existence, or escalate privileges.
// improvement: Validate that the target record exists before using connectOrCreate, or use connect with a pre-verified identifier.

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function assignCategory(productId: string, categoryName: string) {
  return prisma.product.update({
    where: { id: productId },
    data: {
      category: {
        connectOrCreate: {
          where: { name: categoryName },
          create: { name: categoryName }
        }
      }
    }
  });
}
