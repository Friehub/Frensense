// SAFE: Uses visibility-based filtering — only includes docs where user is owner or explicit collaborator

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function exportOrgData(req: Request, res: Response) {
  const { orgId } = req.params;
  const userId = req.user.id;

  const documents = await prisma.document.findMany({
    where: {
      orgId,
      OR: [
        { ownerId: userId },
        { collaborators: { some: { userId } } },
      ],
    },
  });

  const exportData = {
    documents,
    exportedAt: new Date().toISOString(),
  };

  res.json(exportData);
}
