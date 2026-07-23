// [frensense]
// observation: The data export endpoint processes and returns a full data archive without requiring the user to re-authenticate or confirm their password.
// impact: If a user's session is hijacked or left unattended, an attacker can exfiltrate all of the user's data in a single export request.
// improvement: Require re-authentication (password confirmation) before initiating a data export.

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function exportUserData(req: Request, res: Response) {
  const userId = req.user.id;

  const [profile, documents, orders] = await Promise.all([
    prisma.user.findUnique({ where: { id: userId } }),
    prisma.document.findMany({ where: { ownerId: userId } }),
    prisma.order.findMany({ where: { userId } }),
  ]);

  const exportData = {
    profile,
    documents,
    orders,
    exportedAt: new Date().toISOString(),
  };

  res.setHeader('Content-Type', 'application/json');
  res.setHeader('Content-Disposition', 'attachment; filename="data-export.json"');
  res.json(exportData);
}
