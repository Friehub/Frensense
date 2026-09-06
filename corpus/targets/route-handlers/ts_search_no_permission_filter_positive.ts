// [frensense]
// observation: The search endpoint queries across all documents without filtering by the user's access permissions, returning results the user should not be able to see.
// impact: Users can discover confidential documents, internal notes, or other users' private data through search, leading to data leakage.
// improvement: Always scope search queries to only include resources that the requesting user has permission to access.

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function searchDocuments(req: Request, res: Response) {
  const { q, orgId } = req.query;

  const documents = await prisma.document.findMany({
    where: {
      orgId: String(orgId),
      OR: [
        { title: { contains: String(q) } },
        { content: { contains: String(q) } },
      ],
    },
    take: 50,
  });

  res.json(documents);
}
