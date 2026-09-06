// SAFE: Uses Prisma's built-in safe query builder with parameterized search

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function searchProducts(req: Request, res: Response) {
  const { q } = req.query;

  const products = await prisma.product.findMany({
    where: {
      OR: [
        { name: { contains: String(q), mode: 'insensitive' } },
        { description: { contains: String(q), mode: 'insensitive' } },
      ],
    },
    take: 50,
  });

  res.json(products);
}
