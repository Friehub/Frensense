// SAFE: Scopes search to documents the user has access to via membership or collaboration

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function searchDocuments(req: Request, res: Response) {
  const { q, orgId } = req.query;
  const userId = req.user.id;

  const documents = await prisma.document.findMany({
    where: {
      orgId: String(orgId),
      OR: [
        { title: { contains: String(q) } },
        { content: { contains: String(q) } },
      ],
      OR: [
        { ownerId: userId },
        { collaborators: { some: { userId } } },
        { visibility: 'PUBLIC' },
      ],
    },
    take: 50,
  });

  res.json(documents);
}
