// [frensense]
// observation: The filter endpoint returns distinct error messages or status codes that reveal whether a resource exists, even when the user doesn't have access to it.
// impact: Attackers can enumerate hidden resources (users, documents, internal projects) by observing the difference between 'not found' and 'access denied' responses.
// improvement: Return identical error responses for both 'not found' and 'access denied' cases to prevent information leakage.

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function filterDocuments(req: Request, res: Response) {
  const { status, ownerId, projectId } = req.query;
  const userId = req.user.id;

  try {
    const documents = await prisma.document.findMany({
      where: {
        status: String(status),
        ownerId: String(ownerId),
        projectId: String(projectId),
      },
      take: 100,
    });

    if (documents.length === 0) {
      return res.status(404).json({ error: 'No documents found with these filters' });
    }

    res.json(documents);
  } catch (err) {
    if (err.code === 'P2025') {
      return res.status(404).json({ error: 'Project not found' });
    }
    throw err;
  }
}
