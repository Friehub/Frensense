// SAFE: Sort column validated against an explicit allowlist.
import { Request, Response } from 'express';
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

const ALLOWED_SORT_COLUMNS = new Set([
  'createdAt',
  'updatedAt',
  'name',
  'price',
  'stock',
]);

export async function listProducts(req: Request, res: Response): Promise<void> {
  const sortBy = req.query.sortBy as string;

  if (sortBy && !ALLOWED_SORT_COLUMNS.has(sortBy)) {
    res.status(400).json({ error: `invalid sort column: ${sortBy}` });
    return;
  }

  const sortOrder = req.query.sortOrder === 'asc' ? 'asc' : 'desc';
  const products = await prisma.product.findMany({
    orderBy: { [sortBy || 'createdAt']: sortOrder },
  });

  res.json(products);
}
