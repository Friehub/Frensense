// [frensense]
// observation: Sort column parameter from user input is passed directly into SQL `ORDER BY` clause without allowlist validation, enabling blind SQL injection via crafted column names.
// impact: SQL injection in ORDER BY clause — while ORDER BY cannot use UNION, an attacker can use CASE/WHEN with timing to extract data (blind SQLi), or cause errors that leak schema info.
// improvement: Validate sort column against an explicit allowlist of known column names.

import { Request, Response } from 'express';
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function listProducts(req: Request, res: Response): Promise<void> {
  const sortBy = req.query.sortBy as string || 'createdAt';
  const sortOrder = req.query.sortOrder as string || 'desc';

  const products = await prisma.product.findMany({
    orderBy: { [sortBy]: sortOrder },
  });

  res.json(products);
}
