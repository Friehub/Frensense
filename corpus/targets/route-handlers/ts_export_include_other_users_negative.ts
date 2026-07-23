// SAFE: Filters documents by ownerId to only include the requesting user's data

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function exportOrgData(req: Request, res: Response) {
  const { orgId } = req.params;
  const userId = req.user.id;

  const documents = await prisma.document.findMany({
    where: { orgId, ownerId: userId },
  });

  const exportData = {
    documents,
    exportedAt: new Date().toISOString(),
  };

  res.json(exportData);
}
