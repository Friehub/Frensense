// SAFE: Uses a permission-check subquery that verifies user has access before returning results

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function searchDocuments(req: Request, res: Response) {
  const { q, orgId } = req.query;
  const userId = req.user.id;

  const user = await prisma.user.findUnique({
    where: { id: userId },
    include: {
      orgMemberships: { where: { orgId: String(orgId) } },
    },
  });

  if (!user || user.orgMemberships.length === 0) {
    return res.status(403).json({ error: 'Not a member of this organization' });
  }

  const documents = await prisma.document.findMany({
    where: {
      orgId: String(orgId),
      AND: [
        {
          OR: [
            { title: { contains: String(q) } },
            { content: { contains: String(q) } },
          ],
        },
        {
          OR: [
            { ownerId: userId },
            { visibility: 'ORG' },
            { visibility: 'PUBLIC' },
          ],
        },
      ],
    },
    take: 50,
  });

  res.json(documents);
}
