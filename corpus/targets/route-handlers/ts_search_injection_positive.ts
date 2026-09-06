// [frensense]
// observation: The search term from the user is directly interpolated into a database query or search engine query without sanitization or parameterization.
// impact: Attackers can perform SQL injection or NoSQL injection through the search endpoint, manipulating the query to access unauthorized data.
// improvement: Use parameterized queries or an ORM with safe query building for all search operations.

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function searchProducts(req: Request, res: Response) {
  const { q } = req.query;

  const products = await prisma.$queryRawUnsafe(
    `SELECT * FROM products WHERE name ILIKE '%${q}%' OR description ILIKE '%${q}%' LIMIT 50`,
  );

  res.json(products);
}
