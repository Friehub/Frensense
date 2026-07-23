// SAFE: Uses parameterized raw query with Prisma instead of string interpolation

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function searchProducts(req: Request, res: Response) {
  const { q } = req.query;

  const products = await prisma.$queryRaw`
    SELECT * FROM products
    WHERE name ILIKE ${'%' + q + '%'}
       OR description ILIKE ${'%' + q + '%'}
    LIMIT 50
  `;

  res.json(products);
}
