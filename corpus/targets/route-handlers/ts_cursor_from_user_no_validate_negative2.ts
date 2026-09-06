// SAFE: Cursor is base64-decoded and validated as a structured opaque token.
import { Request, Response } from 'express';
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

function decodeCursor(raw: string): { id: string; createdAt: string } | null {
  try {
    const decoded = JSON.parse(Buffer.from(raw, 'base64url').toString('utf-8'));
    if (typeof decoded.id === 'string' && typeof decoded.createdAt === 'string') {
      return decoded;
    }
    return null;
  } catch {
    return null;
  }
}

export async function listUsers(req: Request, res: Response): Promise<void> {
  const cursorRaw = req.query.cursor as string | undefined;
  const limit = Math.min(parseInt(req.query.limit as string, 10) || 50, 100);

  let cursor: { id: string; createdAt: string } | undefined;
  if (cursorRaw) {
    cursor = decodeCursor(cursorRaw);
    if (!cursor) {
      res.status(400).json({ error: 'invalid cursor' });
      return;
    }
  }

  const users = await prisma.user.findMany({
    take: limit,
    skip: cursor ? 1 : 0,
    ...(cursor ? { cursor: { id: cursor.id } } : {}),
    orderBy: { createdAt: 'desc' },
  });

  const nextCursor = users.length === limit
    ? Buffer.from(JSON.stringify({ id: users[users.length - 1].id, createdAt: users[users.length - 1].createdAt })).toString('base64url')
    : null;

  res.json({ users, nextCursor });
}
