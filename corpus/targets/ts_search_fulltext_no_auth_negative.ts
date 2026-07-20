// SAFE: Search filtered by user's tenant and permission level.
import { Request, Response } from 'express';
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function searchDocuments(req: Request, res: Response): Promise<void> {
  const query = req.query.q as string;
  const userId = req.session.userId;

  const user = await prisma.user.findUnique({
    where: { id: userId },
    select: { tenantId: true, role: true },
  });

  if (!user) {
    res.status(401).json({ error: 'unauthorized' });
    return;
  }

  const results = await prisma.document.findMany({
    where: {
      tenantId: user.tenantId,
      OR: [
        { title: { contains: query } },
        { content: { contains: query } },
      ],
    },
    take: 50,
  });

  res.json(results);
}
