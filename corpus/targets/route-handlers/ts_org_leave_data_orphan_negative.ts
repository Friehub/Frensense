// SAFE: Reassigns resources to another org member or restricts access on leave

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function leaveOrganization(req: Request, res: Response) {
  const { orgId } = req.params;
  const userId = req.user.id;

  const ownedDocs = await prisma.document.updateMany({
    where: { ownerId: userId, orgId },
    data: { visibility: 'PRIVATE', ownerId: null },
  });

  await prisma.orgMembership.delete({
    where: { orgId_userId: { orgId, userId } },
  });

  res.json({ message: 'Left organization', documentsRestricted: ownedDocs.count });
}
