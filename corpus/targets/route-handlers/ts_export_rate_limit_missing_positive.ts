// [frensense]
// observation: The data export endpoint has no rate limiting, allowing a user to request unlimited data exports in rapid succession.
// impact: Attackers can exhaust server resources and bandwidth by triggering repeated exports of large datasets, causing denial of service and increased cloud costs.
// improvement: Apply rate limiting to export endpoints and consider queueing large exports as background jobs.

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function exportData(req: Request, res: Response) {
  const userId = req.user.id;

  const data = await prisma.document.findMany({ where: { ownerId: userId } });

  res.setHeader('Content-Type', 'application/json');
  res.setHeader('Content-Disposition', 'attachment; filename="export.json"');
  res.json(data);
}
