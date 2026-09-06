// SAFE: Cursor validation always runs the query — invalid cursors return empty results, not an error.
import { Request, Response } from 'express';
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function listItems(req: Request, res: Response): Promise<void> {
  const cursor = req.query.cursor as string | undefined;

  const items = await prisma.item.findMany({
    take: 50,
    ...(cursor ? { cursor: { id: cursor }, skip: 1 } : {}),
  });

  res.json({
    items,
    nextCursor: items.length === 50 ? items[items.length - 1].id : null,
  });
}
