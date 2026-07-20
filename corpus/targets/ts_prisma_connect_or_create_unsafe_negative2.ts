// SAFE: Allowed categories whitelist prevents creation of arbitrary records

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

const ALLOWED_CATEGORIES = ['electronics', 'clothing', 'food', 'books'];

export async function assignCategory(productId: string, categoryName: string) {
  if (!ALLOWED_CATEGORIES.includes(categoryName)) {
    throw new Error('Category not allowed');
  }
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
