// SAFE: Sort direction validated against explicit allowlist.
import { Request, Response } from 'express';
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

const ALLOWED_SORT_COLUMNS = new Set(['createdAt', 'name', 'email']);

export async function listUsers(req: Request, res: Response): Promise<void> {
  const sortBy = req.query.sortBy as string;
  const sortOrderRaw = req.query.sortOrder as string;

  if (sortBy && !ALLOWED_SORT_COLUMNS.has(sortBy)) {
    res.status(400).json({ error: 'invalid sort column' });
    return;
  }

  const sortOrder = sortOrderRaw === 'asc' ? 'asc' : 'desc';

  const users = await prisma.user.findMany({
    orderBy: { [sortBy || 'createdAt']: sortOrder },
  });

  res.json(users);
}
