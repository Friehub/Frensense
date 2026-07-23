// SAFE: Sort direction parsed as a boolean/ternary rather than interpolated string.
import { Request, Response } from 'express';
import { PrismaClient, Prisma } from '@prisma/client';

const prisma = new PrismaClient();

type SortDir = 'asc' | 'desc';

function parseSortDirection(raw: string | undefined): SortDir {
  if (raw === 'asc') return 'asc';
  return 'desc';
}

export async function listUsers(req: Request, res: Response): Promise<void> {
  const sortDir = parseSortDirection(req.query.sortOrder as string | undefined);

  const orderBy: Prisma.UserOrderByWithRelationInput = {
    createdAt: sortDir,
  };

  const users = await prisma.user.findMany({ orderBy });
  res.json(users);
}
