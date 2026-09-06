// SAFE: Normalizes error responses to prevent distinguishing missing vs inaccessible resources

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function filterDocuments(req: Request, res: Response) {
  const { status, ownerId, projectId } = req.query;
  const userId = req.user.id;

  let documents;
  try {
    documents = await prisma.document.findMany({
      where: {
        status: String(status),
        ownerId: String(ownerId),
        projectId: String(projectId),
        OR: [{ ownerId: userId }, { visibility: 'PUBLIC' }],
      },
      take: 100,
    });
  } catch {
    return res.status(403).json({ error: 'Access denied' });
  }

  res.json(documents);
}
