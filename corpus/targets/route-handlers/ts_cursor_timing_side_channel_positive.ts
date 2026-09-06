// [frensense]
// observation: Cursor-based pagination leaks whether a cursor is valid or invalid through response timing — valid cursors return faster (DB hit) vs invalid cursors (immediate 400).
// impact: Timing side channel allows an attacker to enumerate valid cursor values, enabling data enumeration even when UUIDs are used as cursors.
// improvement: Always respond with identical timing regardless of cursor validity — validate asynchronously or use constant-time checks.

import { Request, Response } from 'express';
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function listItems(req: Request, res: Response): Promise<void> {
  const cursor = req.query.cursor as string;

  if (!cursor || cursor.length < 10) {
    res.status(400).json({ error: 'invalid cursor' });
    return;
  }

  const items = await prisma.item.findMany({
    take: 50,
    ...(cursor ? { cursor: { id: cursor }, skip: 1 } : {}),
  });

  res.json({
    items,
    nextCursor: items.length === 50 ? items[items.length - 1].id : null,
  });
}
