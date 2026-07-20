// SAFE: Category existence verified before connectOrCreate, preventing silent creation

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function assignCategory(productId: string, categoryName: string) {
  const category = await prisma.category.findUnique({ where: { name: categoryName } });
  if (!category) throw new Error('Category not found');
  return prisma.product.update({
    where: { id: productId },
    data: {
      category: { connect: { id: category.id } }
    }
  });
}
