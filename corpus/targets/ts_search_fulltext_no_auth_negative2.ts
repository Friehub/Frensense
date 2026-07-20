// SAFE: Search uses a permission-filtered view or index.
import { Request, Response } from 'express';
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function searchDocuments(req: Request, res: Response): Promise<void> {
  const query = req.query.q as string;
  const userId = req.session.userId;

  const accessibleDocIds = await prisma.documentPermission.findMany({
    where: { userId, granted: true },
    select: { documentId: true },
  });

  const accessibleIds = new Set(accessibleDocIds.map(d => d.documentId));

  const results = await prisma.document.findMany({
    where: {
      id: { in: [...accessibleIds] },
      OR: [
        { title: { contains: query } },
        { content: { contains: query } },
      ],
    },
    take: 50,
  });

  res.json(results);
}
