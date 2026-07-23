// SAFE: Sort column built using Prisma's typed API to avoid dynamic keys.
import { Request, Response } from 'express';
import { PrismaClient, Prisma } from '@prisma/client';

const prisma = new PrismaClient();

const SORT_MAP: Record<string, Prisma.ProductOrderByWithRelationInput> = {
  createdAt: { createdAt: 'desc' },
  createdAtAsc: { createdAt: 'asc' },
  nameAsc: { name: 'asc' },
  nameDesc: { name: 'desc' },
  priceAsc: { price: 'asc' },
  priceDesc: { price: 'desc' },
};

export async function listProducts(req: Request, res: Response): Promise<void> {
  const sortKey = req.query.sort as string || 'createdAt';
  const orderBy = SORT_MAP[sortKey];

  if (!orderBy) {
    res.status(400).json({ error: `invalid sort option: ${sortKey}` });
    return;
  }

  const products = await prisma.product.findMany({ orderBy });
  res.json(products);
}
