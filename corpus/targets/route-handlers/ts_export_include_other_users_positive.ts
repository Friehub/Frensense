// [frensense]
// observation: The data export endpoint includes data from other users in the organization, because the query does not filter by the requesting user's ownership.
// impact: Users can export sensitive data belonging to other organization members, violating data privacy and potentially GDPR data portability rights.
// improvement: Always scope data export queries to only return data owned by or explicitly shared with the requesting user.

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function exportOrgData(req: Request, res: Response) {
  const { orgId } = req.params;
  const userId = req.user.id;

  const documents = await prisma.document.findMany({
    where: { orgId },
  });

  const exportData = {
    documents,
    exportedAt: new Date().toISOString(),
  };

  res.json(exportData);
}
