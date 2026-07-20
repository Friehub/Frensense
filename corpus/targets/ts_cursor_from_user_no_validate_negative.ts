// SAFE: Cursor validated as a UUID before use in database query.
import { Request, Response } from 'express';
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

const UUID_REGEX = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;

export async function listUsers(req: Request, res: Response): Promise<void> {
  const cursor = req.query.cursor as string | undefined;
  const limit = Math.min(parseInt(req.query.limit as string, 10) || 50, 100);

  if (cursor && !UUID_REGEX.test(cursor)) {
    res.status(400).json({ error: 'invalid cursor format' });
    return;
  }

  const users = await prisma.user.findMany({
    take: limit,
    skip: cursor ? 1 : 0,
    ...(cursor ? { cursor: { id: cursor } } : {}),
  });

  res.json({
    users,
    nextCursor: users.length === limit ? users[users.length - 1].id : null,
  });
}
