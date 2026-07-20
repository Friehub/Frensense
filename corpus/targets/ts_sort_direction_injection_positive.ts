// [frensense]
// observation: Sort direction (ASC/DESC) from user input without validation — attacker provides arbitrary values like `'; DROP TABLE products; --` that pass through to `ORDER BY col attackerValue`.
// impact: SQL injection via sort direction — queries like `ORDER BY name attackerControlled` break SQL syntax and can be exploited for UNION-based injection or error-based extraction.
// improvement: Validate sort direction against an explicit allowlist of `asc` and `desc`.

import { Request, Response } from 'express';
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function listUsers(req: Request, res: Response): Promise<void> {
  const sortBy = req.query.sortBy as string || 'createdAt';
  const sortOrder = req.query.sortOrder as string || 'desc';

  const users = await prisma.user.findMany({
    orderBy: { [sortBy]: sortOrder },
  });

  res.json(users);
}
