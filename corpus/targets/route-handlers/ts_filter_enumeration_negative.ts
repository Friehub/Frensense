// SAFE: Returns the same response for both empty results and inaccessible resources

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function filterDocuments(req: Request, res: Response) {
  const { status, ownerId, projectId } = req.query;
  const userId = req.user.id;

  const documents = await prisma.document.findMany({
    where: {
      status: String(status),
      ownerId: String(ownerId),
      projectId: String(projectId),
      OR: [
        { ownerId: userId },
        { visibility: 'PUBLIC' },
      ],
    },
    take: 100,
  });

  res.json(documents);
}
